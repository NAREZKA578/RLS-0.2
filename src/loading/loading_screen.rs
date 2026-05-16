use crate::core::app_state::AppState;
use crate::graphics::ui_renderer::batch::{Color, DrawBatch, Rect};
use super::loader::{BackgroundLoader, LoadedAssets, LoadMsg};
use std::path::PathBuf;

pub struct LoadingScreen {
    loader: Option<BackgroundLoader>,
    current_stage: u8,
    stage_name: &'static str,
    progress: f32,
    total_progress: f32,
    total_stages: u8,
    pub loaded: Option<LoadedAssets>,
}

impl LoadingScreen {
    pub fn new(
        character_data: crate::core::app_state::CharacterData,
        screen_w: f32,
        screen_h: f32,
        assets_dir: PathBuf,
    ) -> Self {
        let loader = BackgroundLoader::start(assets_dir, character_data);
        Self {
            loader: Some(loader),
            current_stage: 0,
            stage_name: "Инициализация...",
            progress: 0.0,
            total_progress: 0.0,
            total_stages: 4,
            loaded: None,
        }
    }

    pub fn update(&mut self, _dt: f32) -> Option<AppState> {
        if let Some(ref loader) = self.loader {
            while let Ok(msg) = loader.rx.try_recv() {
                match msg {
                    LoadMsg::StageBegin { stage, name } => {
                        self.current_stage = stage;
                        self.stage_name = name;
                        self.progress = 0.0;
                    }
                    LoadMsg::Progress(p) => {
                        self.progress = p;
                        self.total_progress = (self.current_stage as f32 - 1.0 + p)
                            / self.total_stages as f32;
                    }
                    LoadMsg::Done(assets) => {
                        self.loaded = Some(*assets);
                        self.loader = None;
                        return Some(AppState::Playing);
                    }
                    LoadMsg::Error(e) => {
                        tracing::error!("Loading failed: {}", e);
                        return Some(AppState::MainMenu);
                    }
                }
            }
        }
        None
    }

    pub fn render(
        &self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        sw: f32,
        sh: f32,
    ) {
        batch.push_rect(
            Rect {
                x: 0.0,
                y: 0.0,
                w: sw,
                h: sh,
            },
            Color::RTGC_BG,
            0.0,
        );

        batch.push_rect(
            Rect {
                x: 0.0,
                y: 0.0,
                w: sw * self.total_progress,
                h: 3.0,
            },
            Color::RTGC_ACCENT,
            0.0,
        );

        let stage_text = format!("{} / {}", self.current_stage, self.total_stages);
        let stage_w = ui.measure_text_width(&stage_text);
        ui.push_text(
            batch,
            &stage_text,
            (sw - stage_w) / 2.0,
            sh / 2.0 - 60.0,
            Color::RTGC_TEXT_DIM,
        );

        let pct_text = format!("{:.0}%", self.total_progress * 100.0);
        let pct_w = ui.measure_text_width(&pct_text);
        ui.push_text(
            batch,
            &pct_text,
            (sw - pct_w) / 2.0,
            sh / 2.0 - 36.0,
            Color::WHITE,
        );

        let dots = ".".repeat(((self.progress * 4.0) as usize % 4) + 1);
        let text = format!("ЗАГРУЗКА{}", dots);
        let text_w = ui.measure_text_width(&text);
        ui.push_text(batch, &text, (sw - text_w) / 2.0, sh / 2.0, Color::WHITE);

        ui.push_text(
            batch,
            self.stage_name,
            (sw - ui.measure_text_width(self.stage_name)) / 2.0,
            sh / 2.0 + 30.0,
            Color::RTGC_TEXT_DIM,
        );
    }
}