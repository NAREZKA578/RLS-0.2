use crate::core::app_state::AppState;
use crate::graphics::ui_renderer::batch::{Color, DrawBatch, Rect};
use crate::ui::button::Button;
use crate::ui::panel::Panel;

pub struct SaveSelectScreen {
    back_btn: Button,
    hovered_back: bool,
    _hovered_slot: Option<usize>,
}

impl SaveSelectScreen {
    pub fn new(sw: f32, sh: f32) -> Self {
        let btn_w = 200.0;
        let btn_h = 48.0;
        Self {
            back_btn: Button::new((sw - btn_w) / 2.0, sh - 80.0, btn_w, btn_h, "НАЗАД"),
            hovered_back: false,
            _hovered_slot: None,
        }
    }

    pub fn update(
        &mut self,
        mouse_x: f32,
        mouse_y: f32,
        mouse_just_pressed: bool,
    ) -> Option<AppState> {
        self.hovered_back = self.back_btn.update(mouse_x, mouse_y, mouse_just_pressed);
        if self.hovered_back {
            return Some(AppState::MainMenu);
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
            Color::new(0.1, 0.1, 0.14, 1.0),
            0.0,
        );

        let title = "ЗАГРУЗИТЬ ИГРУ";
        let tw = ui.measure_text_width(title);
        ui.push_text(
            batch,
            title,
            (sw - tw) / 2.0,
            sh / 2.0 - 166.0,
            Color::WHITE,
        );

        let panel_w = 500.0;
        let panel_h = 350.0;
        Panel::new(
            (sw - panel_w) / 2.0,
            (sh - panel_h) / 2.0 - 30.0,
            panel_w,
            panel_h,
            Color::new(0.15, 0.15, 0.2, 0.95),
        )
        .with_corner_radius(8.0)
        .render(batch);

        let hint = "Нет сохранений";
        let hint_w = ui.measure_text_width(hint);
        ui.push_text(
            batch,
            hint,
            (sw - hint_w) / 2.0,
            sh / 2.0,
            Color::new(0.45, 0.45, 0.5, 1.0),
        );

        let hint2 = "Создайте новую игру";
        let hint2_w = ui.measure_text_width(hint2);
        ui.push_text(
            batch,
            hint2,
            (sw - hint2_w) / 2.0,
            sh / 2.0 + 30.0,
            Color::new(0.35, 0.35, 0.4, 1.0),
        );

        self.back_btn.render(batch);

        let btn_text = "НАЗАД";
        let text_w = ui.measure_text_width(btn_text);
        let br = &self.back_btn.rect;
        ui.push_text(
            batch,
            btn_text,
            br.x + (br.w - text_w) / 2.0,
            br.y + br.h / 2.0 + 4.0,
            Color::WHITE,
        );
    }
}
