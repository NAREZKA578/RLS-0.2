use crate::core::app_state::{AppState, CharacterData};
use crate::graphics::ui_renderer::batch::{Color, DrawBatch, Rect};
use crate::ui::progress_bar::ProgressBar;

pub struct LoadingScreen {
    pub _character_data: CharacterData,
    progress_bar: ProgressBar,
    current_stage: u8,
    total_stages: u8,
    stage_progress: f32,
}

impl LoadingScreen {
    pub fn new(_character_data: CharacterData, screen_w: f32, screen_h: f32) -> Self {
        let bar_w = 500.0;
        let bar_h = 18.0;
        let bar_x = (screen_w - bar_w) / 2.0;
        let bar_y = screen_h / 2.0 + 50.0;
        let mut bar = ProgressBar::new(bar_x, bar_y, bar_w, bar_h);
        bar.fill_color = Color::new(0.85, 0.55, 0.15, 1.0);
        Self {
            _character_data,
            progress_bar: bar,
            current_stage: 1,
            total_stages: 11,
            stage_progress: 0.0,
        }
    }

    pub fn stage_message(&self) -> &'static str {
        match self.current_stage {
            1 => "Загрузка шрифтов",
            2 => "Загрузка текстур",
            3 => "Инициализация рендерера",
            4 => "Генерация дорожной сети",
            5 => "Загрузка ландшафта",
            6 => "Инициализация физики",
            7 => "Загрузка звуков",
            8 => "Создание персонажа",
            9 => "Спавн техники",
            10 => "Подготовка мира",
            11 => "Финальная настройка",
            _ => "Загрузка...",
        }
    }

    pub fn update(&mut self, dt: f32) -> Option<AppState> {
        self.stage_progress += dt * 50.0;

        if self.stage_progress >= 1.0 {
            self.current_stage += 1;
            self.stage_progress = 0.0;

            if self.current_stage > self.total_stages {
                return Some(AppState::Playing);
            }
        }

        let total_progress = ((self.current_stage as f32 - 1.0 + self.stage_progress)
            / self.total_stages as f32)
            .clamp(0.0, 1.0);
        self.progress_bar.set_progress(total_progress);

        None
    }

    pub fn render(
        &self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        _sw: f32,
        _sh: f32,
    ) {
        batch.push_rect(
            Rect {
                x: 0.0,
                y: 0.0,
                w: _sw,
                h: _sh,
            },
            Color::RTGC_BG,
            0.0,
        );

        batch.push_rect(
            Rect {
                x: 0.0,
                y: 0.0,
                w: _sw * self.progress_bar.get_progress(),
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
            (_sw - stage_w) / 2.0,
            _sh / 2.0 - 60.0,
            Color::RTGC_TEXT_DIM,
        );

        let pct_text = format!("{:.0}%", self.progress_bar.get_progress() * 100.0);
        let pct_w = ui.measure_text_width(&pct_text);
        ui.push_text(
            batch,
            &pct_text,
            (_sw - pct_w) / 2.0,
            _sh / 2.0 - 36.0,
            Color::WHITE,
        );

        let dots = ".".repeat(((self.stage_progress * 4.0) as usize % 4) + 1);
        let text = format!("ЗАГРУЗКА{}", dots);
        let text_w = ui.measure_text_width(&text);
        ui.push_text(batch, &text, (_sw - text_w) / 2.0, _sh / 2.0, Color::WHITE);

        let msg = self.stage_message();
        ui.push_text(
            batch,
            msg,
            (_sw - ui.measure_text_width(msg)) / 2.0,
            _sh / 2.0 + 30.0,
            Color::RTGC_TEXT_DIM,
        );

        self.progress_bar.render(batch);
    }
}
