use crate::animation::tween::Tween;
use crate::core::app_state::AppState;
use crate::graphics::ui_renderer::batch::{Color, DrawBatch, Rect};
use crate::platform::input::InputState;
use winit::keyboard::KeyCode;

pub struct SplashScreen {
    logo_alpha: Tween,
    elapsed: f32,
    duration: f32,
}

impl Default for SplashScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl SplashScreen {
    pub fn new() -> Self {
        Self {
            logo_alpha: Tween::new(0.0, 1.0, 1.5)
                .with_easing(crate::animation::easing::ease_out_cubic),
            elapsed: 0.0,
            duration: 2.5,
        }
    }

    pub fn update(&mut self, dt: f32, input: &InputState) -> Option<AppState> {
        self.elapsed += dt;
        self.logo_alpha.update(dt);

        let skip = input.is_mouse_button_just_pressed(0)
            || input.is_key_just_pressed(KeyCode::Space)
            || input.is_key_just_pressed(KeyCode::Enter)
            || input.is_key_just_pressed(KeyCode::Escape);

        if self.elapsed >= self.duration || (self.elapsed > 0.5 && skip) {
            Some(AppState::MainMenu)
        } else {
            None
        }
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

        let alpha = self.logo_alpha.value();

        batch.push_rect(
            Rect {
                x: (sw - 300.0) / 2.0,
                y: sh / 2.0 - 75.0,
                w: 300.0,
                h: 60.0,
            },
            Color::RTGC_ACCENT.with_alpha(alpha * 0.12),
            8.0,
        );

        let t = "RTGC";
        ui.push_text(
            batch,
            t,
            (sw - ui.measure_text_width(t)) / 2.0,
            sh / 2.0 - 18.0,
            Color::RTGC_ACCENT.with_alpha(alpha),
        );

        if self.elapsed > 0.8 {
            let blink = (self.elapsed % 1.4) < 0.8;
            if blink {
                let hint = "Нажмите любую клавишу";
                ui.push_text(
                    batch,
                    hint,
                    (sw - ui.measure_text_width(hint)) / 2.0,
                    sh / 2.0 + 90.0,
                    Color::RTGC_TEXT_DIM.with_alpha(alpha * 0.8),
                );
            }
        }
    }
}
