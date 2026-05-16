use crate::animation::easing;
use crate::animation::tween::Tween;
use crate::core::app_state::AppState;
use crate::graphics::ui_renderer::batch::{Color, DrawBatch, Rect};
use crate::ui::button::Button;

pub enum MenuResult {
    NewGame,
    LoadGame,
    Settings,
    Exit,
}

struct BtnCfg {
    w: f32,
    h: f32,
}

const BTN_CFGS: [BtnCfg; 3] = [
    BtnCfg { w: 320.0, h: 54.0 },
    BtnCfg { w: 280.0, h: 46.0 },
    BtnCfg { w: 280.0, h: 46.0 },
];

pub struct MainMenuScreen {
    logo_alpha: Tween,
    buttons: [Button; 3],
    buttons_alpha: [Tween; 3],
    button_y_offsets: [Tween; 3],
    button_delays: [f32; 3],
    elapsed: f32,
    hovered_btn: Option<usize>,
    hovered_exit: bool,
    exit_rect: Rect,
    pub result: Option<MenuResult>,
}

impl MainMenuScreen {
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        let cx = screen_w / 2.0;
        let btn_start_y = screen_h / 2.0 - 60.0;
        let labels = ["НОВАЯ ИГРА", "ЗАГРУЗИТЬ ИГРУ", "НАСТРОЙКИ"];
        let y = btn_start_y;
        let buttons = std::array::from_fn(|i| {
            let cfg = &BTN_CFGS[i];
            let btn = Button::new(cx - cfg.w / 2.0, y, cfg.w, cfg.h, labels[i]);
            if i == 0 {
                let mut b = btn;
                b.corner_radius = 6.0;
                b
            } else {
                btn
            }
        });

        Self {
            logo_alpha: Tween::new(0.0, 1.0, 0.8).with_easing(easing::ease_out_cubic),
            buttons,
            buttons_alpha: [
                Tween::new(0.0, 1.0, 0.5).with_easing(easing::ease_out_cubic),
                Tween::new(0.0, 1.0, 0.5).with_easing(easing::ease_out_cubic),
                Tween::new(0.0, 1.0, 0.5).with_easing(easing::ease_out_cubic),
            ],
            button_y_offsets: [
                Tween::new(20.0, 0.0, 0.4).with_easing(easing::ease_out_cubic),
                Tween::new(20.0, 0.0, 0.4).with_easing(easing::ease_out_cubic),
                Tween::new(20.0, 0.0, 0.4).with_easing(easing::ease_out_cubic),
            ],
            button_delays: [0.0, 0.08, 0.16],
            elapsed: 0.0,
            hovered_btn: None,
            hovered_exit: false,
            exit_rect: Rect {
                x: 0.0,
                y: 0.0,
                w: 80.0,
                h: 24.0,
            },
            result: None,
        }
    }

    fn compute_button_rects(&self, sw: f32, sh: f32) -> Vec<Rect> {
        let cx = sw / 2.0;
        let btn_start_y = sh / 2.0 - 60.0;
        let mut rects = Vec::with_capacity(3);
        let mut y = btn_start_y;
        for (i, cfg) in BTN_CFGS.iter().enumerate().take(3) {
            let y_offset = if self.elapsed > self.button_delays[i] {
                self.button_y_offsets[i].value()
            } else {
                20.0
            };
            rects.push(Rect {
                x: cx - cfg.w / 2.0,
                y: y + y_offset,
                w: cfg.w,
                h: cfg.h,
            });
            y += cfg.h + 16.0;
        }
        rects
    }

    fn compute_exit_rect(&self, sw: f32, sh: f32) -> Rect {
        let cx = sw / 2.0;
        let exit_y = sh / 2.0 + 130.0;
        Rect {
            x: cx - 40.0,
            y: exit_y - 10.0,
            w: 80.0,
            h: 24.0,
        }
    }

    pub fn update(
        &mut self,
        mouse_x: f32,
        mouse_y: f32,
        mouse_just_pressed: bool,
        dt: f32,
        screen_w: f32,
        screen_h: f32,
    ) -> Option<AppState> {
        self.elapsed += dt;
        self.logo_alpha.update(dt);

        for i in 0..3 {
            if self.elapsed >= self.button_delays[i] {
                let adj_dt = if self.elapsed - dt < self.button_delays[i] {
                    self.elapsed - self.button_delays[i]
                } else {
                    dt
                };
                self.buttons_alpha[i].update(adj_dt);
                self.button_y_offsets[i].update(adj_dt);
            }
        }

        let rects = self.compute_button_rects(screen_w, screen_h);

        self.hovered_btn = None;
        for (i, rect) in rects.iter().enumerate().take(3) {
            if self.elapsed >= self.button_delays[i] {
                self.buttons[i].rect = *rect;
            }
            if self.buttons[i].update(mouse_x, mouse_y, mouse_just_pressed) {
                return match i {
                    0 => Some(AppState::CharacterCreation),
                    1 => Some(AppState::SaveSelect),
                    2 => Some(AppState::Settings {
                        return_to: Box::new(AppState::MainMenu),
                    }),
                    _ => None,
                };
            }
            if rects[i].contains(mouse_x, mouse_y) {
                self.hovered_btn = Some(i);
            }
        }

        self.exit_rect = self.compute_exit_rect(screen_w, screen_h);
        self.hovered_exit = self.exit_rect.contains(mouse_x, mouse_y);
        if self.hovered_exit && mouse_just_pressed {
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
        let cx = sw / 2.0;

        batch.push_rect(
            Rect {
                x: 0.0,
                y: 0.0,
                w: sw,
                h: sh * 0.72,
            },
            Color::RTGC_BG_TOP,
            0.0,
        );
        batch.push_rect(
            Rect {
                x: 0.0,
                y: sh * 0.72,
                w: sw,
                h: sh * 0.28,
            },
            Color::RTGC_BG_BOT,
            0.0,
        );
        batch.push_rect(
            Rect {
                x: cx - 200.0,
                y: 0.0,
                w: 400.0,
                h: sh,
            },
            Color::RTGC_CENTER_GLOW,
            0.0,
        );

        let logo_alpha = self.logo_alpha.value().clamp(0.0, 1.0);
        let lx = cx - 80.0;
        let ly = sh / 2.0 - 190.0;
        let accent = Color::RTGC_ACCENT.with_alpha(logo_alpha);

        batch.push_rect(
            Rect {
                x: lx,
                y: ly,
                w: 8.0,
                h: 52.0,
            },
            accent,
            0.0,
        );
        batch.push_rect(
            Rect {
                x: lx,
                y: ly + 22.0,
                w: 44.0,
                h: 8.0,
            },
            accent,
            0.0,
        );
        batch.push_rect(
            Rect {
                x: lx + 36.0,
                y: ly,
                w: 8.0,
                h: 30.0,
            },
            accent,
            0.0,
        );

        let rlg_x = cx - 26.0;
        let rlg_y = ly + 40.0;
        ui.push_text(batch, "RLG", rlg_x, rlg_y, accent);

        let sub = "Russian Life Game";
        let sub_w = ui.measure_text_width(sub);
        ui.push_text(
            batch,
            sub,
            cx - sub_w / 2.0,
            sh / 2.0 - 120.0,
            Color::RTGC_DIM_TEXT.with_alpha(logo_alpha * 0.8),
        );

        for i in 0..3 {
            let rect = &self.buttons[i].rect;
            let alpha = if self.elapsed >= self.button_delays[i] {
                self.buttons_alpha[i].value().clamp(0.0, 1.0)
            } else {
                0.0
            };
            let is_hovered = self.hovered_btn == Some(i);

            if i == 0 {
                let border_pulse = 0.4 + (self.elapsed * 2.0).sin() * 0.2;
                let border_alpha = if is_hovered { 0.7 } else { border_pulse };
                let border_color = Color::RTGC_ACCENT.with_alpha(border_alpha * alpha);
                batch.push_rect(
                    Rect {
                        x: rect.x - 2.0,
                        y: rect.y - 2.0,
                        w: rect.w + 4.0,
                        h: rect.h + 4.0,
                    },
                    border_color,
                    8.0,
                );
            }

            self.buttons[i].render(batch);

            let tw = ui.measure_text_width(&self.buttons[i].text);
            let tx = rect.x + (rect.w - tw) / 2.0;
            let ty = rect.y + (rect.h - 20.0) / 2.0;
            ui.push_text(
                batch,
                &self.buttons[i].text,
                tx,
                ty,
                Color::RTGC_TEXT.with_alpha(alpha),
            );
        }

        let divider_y = sh / 2.0 + 115.0;
        batch.push_rect(
            Rect {
                x: cx - 180.0,
                y: divider_y,
                w: 360.0,
                h: 1.0,
            },
            Color::RTGC_ACCENT.with_alpha(0.25),
            0.0,
        );

        let exit_c = if self.hovered_exit {
            Color::RTGC_ACCENT.with_alpha(0.8)
        } else {
            Color::RTGC_DIM_TEXT.with_alpha(0.6)
        };
        let exit_y = sh / 2.0 + 130.0;
        let exit_w = ui.measure_text_width("ВЫХОД");
        ui.push_text(batch, "ВЫХОД", cx - exit_w / 2.0, exit_y + 6.0, exit_c);

        let (vx, vy) = self.get_version_pos(sw, sh);
        ui.push_text(
            batch,
            "v1.0.0-dev",
            vx,
            vy,
            Color::RTGC_DIM_TEXT.with_alpha(0.35),
        );
    }

    pub fn get_button_rects(&self, sw: f32, sh: f32) -> Vec<Rect> {
        self.compute_button_rects(sw, sh)
    }

    pub fn get_exit_rect(&self) -> Rect {
        self.exit_rect
    }

    pub fn is_exit_hovered(&self) -> bool {
        self.hovered_exit
    }

    pub fn hovered_button(&self) -> Option<usize> {
        self.hovered_btn
    }

    pub fn get_exit_text_pos(&self, sw: f32, sh: f32) -> (f32, f32) {
        let cx = sw / 2.0;
        let exit_y = sh / 2.0 + 130.0;
        (cx - 20.0, exit_y + 6.0)
    }

    pub fn get_version_pos(&self, sw: f32, sh: f32) -> (f32, f32) {
        let tw = 80.0;
        let exit_y = sh / 2.0 + 130.0;
        (sw / 2.0 - tw / 2.0, exit_y + 50.0)
    }

    pub fn get_copyright_pos(&self, sw: f32, sh: f32) -> (f32, f32) {
        let exit_y = sh / 2.0 + 130.0;
        (sw / 2.0 + 60.0, exit_y + 50.0)
    }
}
