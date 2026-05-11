use crate::core::app_state::AppState;
use crate::graphics::ui_renderer::batch::{Color, DrawBatch, Rect};
use crate::ui::button::Button;
use crate::ui::panel::Panel;
use crate::ui::selector::Selector;
use crate::ui::slider::Slider;

#[derive(Clone, Copy, PartialEq)]
enum SettingsTab {
    Graphics,
    Sound,
    Language,
    Controls,
}

pub struct SettingsScreen {
    return_to: AppState,
    back_btn: Button,
    hovered_back: bool,
    active_tab: SettingsTab,
    tab_buttons: Vec<Rect>,
    hovered_tab: Option<usize>,

    master_volume: Slider,
    music_volume: Slider,
    sfx_volume: Slider,

    resolution: Selector,
    fullscreen: bool,
    vsync: bool,

    language: Selector,
}

impl SettingsScreen {
    pub fn new(return_to: AppState, sw: f32, sh: f32) -> Self {
        let btn_w = 200.0;
        let btn_h = 48.0;

        Self {
            return_to,
            back_btn: Button::new((sw - btn_w) / 2.0, sh - 80.0, btn_w, btn_h, "НАЗАД"),
            hovered_back: false,
            active_tab: SettingsTab::Sound,
            tab_buttons: Vec::new(),
            hovered_tab: None,

            master_volume: Slider::new(0.0, 0.0, 400.0, 8.0, 0.0, 100.0, 80.0),
            music_volume: Slider::new(0.0, 0.0, 400.0, 8.0, 0.0, 100.0, 70.0),
            sfx_volume: Slider::new(0.0, 0.0, 400.0, 8.0, 0.0, 100.0, 100.0),

            resolution: Selector::new(
                0.0,
                0.0,
                400.0,
                vec![
                    "1920x1080".to_string(),
                    "1600x900".to_string(),
                    "1366x768".to_string(),
                    "1280x720".to_string(),
                ],
            ),
            fullscreen: false,
            vsync: true,

            language: Selector::new(
                0.0,
                0.0,
                400.0,
                vec!["Русский".to_string(), "English".to_string()],
            ),
        }
    }

    fn tab_labels() -> Vec<(&'static str, SettingsTab)> {
        vec![
            ("ГРАФИКА", SettingsTab::Graphics),
            ("ЗВУК", SettingsTab::Sound),
            ("ЯЗЫК", SettingsTab::Language),
            ("КЛАВИШИ", SettingsTab::Controls),
        ]
    }

    pub fn update(
        &mut self,
        mouse_x: f32,
        mouse_y: f32,
        mouse_just_pressed: bool,
        mouse_held: bool,
        screen_w: f32,
        screen_h: f32,
    ) -> Option<AppState> {
        let mouse_pressed = mouse_held || mouse_just_pressed;

        self.hovered_tab = None;
        for (i, tab) in self.tab_buttons.iter().enumerate() {
            if tab.contains(mouse_x, mouse_y) {
                self.hovered_tab = Some(i);
            }
        }

        if mouse_just_pressed {
            if let Some(idx) = self.hovered_tab {
                let tabs = Self::tab_labels();
                if idx < tabs.len() {
                    self.active_tab = tabs[idx].1;
                }
            }
        }

        let panel_x = (screen_w - 700.0) / 2.0;
        let panel_y = (screen_h - 500.0) / 2.0 - 20.0;
        let content_y = panel_y + 110.0;
        let slider_w = 500.0;

        match self.active_tab {
            SettingsTab::Sound => {
                self.master_volume.rect.x = panel_x + 100.0;
                self.master_volume.rect.y = content_y + 20.0;
                self.master_volume.rect.w = slider_w;
                self.music_volume.rect.x = panel_x + 100.0;
                self.music_volume.rect.y = content_y + 90.0;
                self.music_volume.rect.w = slider_w;
                self.sfx_volume.rect.x = panel_x + 100.0;
                self.sfx_volume.rect.y = content_y + 160.0;
                self.sfx_volume.rect.w = slider_w;

                self.master_volume.update(mouse_x, mouse_y, mouse_pressed);
                self.music_volume.update(mouse_x, mouse_y, mouse_pressed);
                self.sfx_volume.update(mouse_x, mouse_y, mouse_pressed);
            }
            SettingsTab::Graphics => {
                self.resolution.x = panel_x + 100.0;
                self.resolution.y = content_y + 20.0;
                self.resolution.width = slider_w;

                if mouse_just_pressed {
                    let r = Rect {
                        x: self.resolution.x,
                        y: self.resolution.y,
                        w: self.resolution.width,
                        h: 44.0,
                    };
                    if r.contains(mouse_x, mouse_y) {
                        self.resolution.next();
                    }

                    let fs_r = Rect {
                        x: panel_x + 100.0,
                        y: content_y + 90.0,
                        w: 24.0,
                        h: 24.0,
                    };
                    if fs_r.contains(mouse_x, mouse_y) {
                        self.fullscreen = !self.fullscreen;
                    }

                    let vsync_r = Rect {
                        x: panel_x + 100.0,
                        y: content_y + 140.0,
                        w: 24.0,
                        h: 24.0,
                    };
                    if vsync_r.contains(mouse_x, mouse_y) {
                        self.vsync = !self.vsync;
                    }
                }
            }
            SettingsTab::Language => {
                self.language.x = panel_x + 100.0;
                self.language.y = content_y + 20.0;
                self.language.width = slider_w;

                if mouse_just_pressed {
                    let r = Rect {
                        x: self.language.x,
                        y: self.language.y,
                        w: self.language.width,
                        h: 44.0,
                    };
                    if r.contains(mouse_x, mouse_y) {
                        self.language.next();
                    }
                }
            }
            SettingsTab::Controls => {}
        }

        self.hovered_back = self.back_btn.update(mouse_x, mouse_y, mouse_just_pressed);
        if self.hovered_back {
            return Some(self.return_to.clone());
        }
        None
    }

    pub fn render(
        &mut self,
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
            Color::new(0.08, 0.08, 0.12, 1.0),
            0.0,
        );

        batch.push_rect(
            Rect {
                x: 0.0,
                y: 0.0,
                w: sw,
                h: sh,
            },
            Color::new(0.0, 0.0, 0.0, 0.55),
            0.0,
        );

        let panel_w = 700.0;
        let panel_h = 500.0;
        let panel_x = (sw - panel_w) / 2.0;
        let panel_y = (sh - panel_h) / 2.0 - 20.0;
        Panel::new(
            panel_x,
            panel_y,
            panel_w,
            panel_h,
            Color::new(0.12, 0.12, 0.18, 0.95),
        )
        .with_corner_radius(10.0)
        .render(batch);

        let title = "НАСТРОЙКИ";
        let tw = ui.measure_text_width(title);
        ui.push_text(batch, title, (sw - tw) / 2.0, panel_y + 24.0, Color::WHITE);

        self.render_tabs(batch, ui, panel_x, panel_y + 50.0, panel_w);

        let content_y = panel_y + 110.0;
        match self.active_tab {
            SettingsTab::Sound => self.render_sound_tab(batch, ui, panel_x, content_y),
            SettingsTab::Graphics => self.render_graphics_tab(batch, ui, panel_x, content_y),
            SettingsTab::Language => self.render_language_tab(batch, ui, panel_x, content_y),
            SettingsTab::Controls => self.render_controls_tab(
                batch,
                ui,
                panel_x,
                content_y,
                ui.measure_text_width("WASD / СТРЕЛКИ"),
            ),
        }

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

    fn render_tabs(
        &mut self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        panel_x: f32,
        y: f32,
        panel_w: f32,
    ) {
        let tabs = Self::tab_labels();
        let tab_count = tabs.len();
        let tab_w = (panel_w - 40.0) / tab_count as f32;
        let tab_h = 40.0;
        let gap = 0.0;

        self.tab_buttons.clear();
        for (i, (label, _)) in tabs.iter().enumerate() {
            let x = panel_x + 20.0 + i as f32 * (tab_w + gap);
            self.tab_buttons.push(Rect {
                x,
                y,
                w: tab_w,
                h: tab_h,
            });

            let is_active = self.active_tab == tabs[i].1;
            let is_hovered = self.hovered_tab == Some(i);

            let bg = if is_active {
                Color::new(0.25, 0.25, 0.35, 1.0)
            } else if is_hovered {
                Color::new(0.2, 0.2, 0.28, 1.0)
            } else {
                Color::new(0.15, 0.15, 0.22, 0.8)
            };
            batch.push_rect(
                Rect {
                    x,
                    y,
                    w: tab_w,
                    h: tab_h,
                },
                bg,
                6.0,
            );

            let lw = ui.measure_text_width(label);
            let text_color = if is_active {
                Color::WHITE
            } else {
                Color::new(0.5, 0.5, 0.6, 1.0)
            };
            ui.push_text(
                batch,
                label,
                x + (tab_w - lw) / 2.0,
                y + (tab_h - 20.0) / 2.0 + 5.0,
                text_color,
            );
        }
    }

    fn render_sound_tab(
        &self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        panel_x: f32,
        content_y: f32,
    ) {
        let slider_w = 500.0;
        self.render_slider_row(
            batch,
            ui,
            "ОБЩАЯ ГРОМКОСТЬ",
            panel_x + 100.0,
            content_y + 20.0,
            slider_w,
        );
        self.master_volume.render(batch);

        self.render_slider_row(
            batch,
            ui,
            "МУЗЫКА",
            panel_x + 100.0,
            content_y + 90.0,
            slider_w,
        );
        self.music_volume.render(batch);

        self.render_slider_row(
            batch,
            ui,
            "ЗВУКОВЫЕ ЭФФЕКТЫ",
            panel_x + 100.0,
            content_y + 160.0,
            slider_w,
        );
        self.sfx_volume.render(batch);
    }

    fn render_graphics_tab(
        &self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        panel_x: f32,
        content_y: f32,
    ) {
        let slider_w = 500.0;

        ui.push_text(
            batch,
            "РАЗРЕШЕНИЕ ЭКРАНА",
            panel_x + 100.0,
            content_y,
            Color::new(0.6, 0.6, 0.65, 1.0),
        );
        self.render_selector(
            batch,
            ui,
            self.resolution.x,
            self.resolution.y,
            slider_w,
            self.resolution.current(),
        );

        ui.push_text(
            batch,
            "ПОЛНОЭКРАННЫЙ РЕЖИМ",
            panel_x + 100.0,
            content_y + 70.0,
            Color::new(0.6, 0.6, 0.65, 1.0),
        );
        let fs_r = Rect {
            x: panel_x + 100.0,
            y: content_y + 90.0,
            w: 24.0,
            h: 24.0,
        };
        let fs_bg = if self.fullscreen {
            Color::new(0.2, 0.5, 0.7, 1.0)
        } else {
            Color::new(0.25, 0.25, 0.3, 1.0)
        };
        batch.push_rect(fs_r, fs_bg, 4.0);
        if self.fullscreen {
            let check = Rect {
                x: fs_r.x + 6.0,
                y: fs_r.y + 6.0,
                w: 12.0,
                h: 12.0,
            };
            batch.push_rect(check, Color::new(0.9, 0.75, 0.15, 1.0), 2.0);
        }
        ui.push_text(
            batch,
            if self.fullscreen {
                "ВКЛ"
            } else {
                "ВЫКЛ"
            },
            panel_x + 140.0,
            content_y + 95.0,
            Color::WHITE,
        );

        ui.push_text(
            batch,
            "ВЕРТИКАЛЬНАЯ СИНХРОНИЗАЦИЯ",
            panel_x + 100.0,
            content_y + 130.0,
            Color::new(0.6, 0.6, 0.65, 1.0),
        );
        let vsync_r = Rect {
            x: panel_x + 100.0,
            y: content_y + 150.0,
            w: 24.0,
            h: 24.0,
        };
        let vsync_bg = if self.vsync {
            Color::new(0.2, 0.5, 0.7, 1.0)
        } else {
            Color::new(0.25, 0.25, 0.3, 1.0)
        };
        batch.push_rect(vsync_r, vsync_bg, 4.0);
        if self.vsync {
            let check = Rect {
                x: vsync_r.x + 6.0,
                y: vsync_r.y + 6.0,
                w: 12.0,
                h: 12.0,
            };
            batch.push_rect(check, Color::new(0.9, 0.75, 0.15, 1.0), 2.0);
        }
        ui.push_text(
            batch,
            if self.vsync { "ВКЛ" } else { "ВЫКЛ" },
            panel_x + 140.0,
            content_y + 155.0,
            Color::WHITE,
        );

        ui.push_text(
            batch,
            "КАЧЕСТВО ТЕКСТУР",
            panel_x + 100.0,
            content_y + 190.0,
            Color::new(0.6, 0.6, 0.65, 1.0),
        );
        let quality_r = Rect {
            x: panel_x + 100.0,
            y: content_y + 210.0,
            w: slider_w,
            h: 36.0,
        };
        batch.push_rect(quality_r, Color::new(0.15, 0.15, 0.22, 0.9), 6.0);
        let quality_items = ["НИЗКОЕ", "СРЕДНЕЕ", "ВЫСОКОЕ"];
        let quality_text = quality_items[1];
        let qw = ui.measure_text_width(quality_text);
        ui.push_text(
            batch,
            quality_text,
            quality_r.x + (quality_r.w - qw) / 2.0,
            quality_r.y + 10.0,
            Color::WHITE,
        );
    }

    fn render_language_tab(
        &self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        panel_x: f32,
        content_y: f32,
    ) {
        let slider_w = 500.0;

        ui.push_text(
            batch,
            "ЯЗЫК ИНТЕРФЕЙСА",
            panel_x + 100.0,
            content_y,
            Color::new(0.6, 0.6, 0.65, 1.0),
        );
        self.render_selector(
            batch,
            ui,
            self.language.x,
            self.language.y,
            slider_w,
            self.language.current(),
        );

        let hint = "Изменение языка вступит в силу после перезапуска";
        let hw = ui.measure_text_width(hint);
        ui.push_text(
            batch,
            hint,
            panel_x + 700.0 / 2.0 - hw / 2.0,
            content_y + 80.0,
            Color::new(0.35, 0.35, 0.4, 1.0),
        );
    }

    fn render_controls_tab(
        &self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        panel_x: f32,
        content_y: f32,
        _max_key_w: f32,
    ) {
        let row_h = 50.0;
        let key_w = 120.0;
        let row_gap = 10.0;
        let slider_w = 500.0;

        let controls = [
            ("ВПЕРЕД", "W"),
            ("НАЗАД", "S"),
            ("ВЛЕВО", "A"),
            ("ВПРАВО", "D"),
            ("АКТИВИРОВАТЬ", "E"),
            ("ИНВЕНТАРЬ", "I"),
            ("ПАУЗА", "ESC"),
            ("МЕНЮ", "M"),
        ];

        for (i, (action, key)) in controls.iter().enumerate() {
            let row_y = content_y + i as f32 * (row_h + row_gap);

            ui.push_text(
                batch,
                action,
                panel_x + 60.0,
                row_y + 15.0,
                Color::new(0.6, 0.6, 0.65, 1.0),
            );

            let key_r = Rect {
                x: panel_x + slider_w,
                y: row_y,
                w: key_w,
                h: row_h,
            };
            batch.push_rect(key_r, Color::new(0.18, 0.18, 0.24, 1.0), 6.0);
            let kw = ui.measure_text_width(key);
            ui.push_text(
                batch,
                key,
                key_r.x + (key_r.w - kw) / 2.0,
                row_y + 15.0,
                Color::WHITE,
            );
        }
    }

    fn render_selector(
        &self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        x: f32,
        y: f32,
        w: f32,
        value: &str,
    ) {
        let r = Rect { x, y, w, h: 44.0 };
        batch.push_rect(r, Color::new(0.15, 0.15, 0.22, 0.9), 6.0);

        let vw = ui.measure_text_width(value);
        ui.push_text(batch, value, x + (w - vw) / 2.0, y + 12.0, Color::WHITE);

        let arrow_x = x + w - 30.0;
        let arrow_color = Color::new(0.5, 0.5, 0.6, 1.0);
        ui.push_text(batch, "▼", arrow_x, y + 10.0, arrow_color);
    }

    fn render_slider_row(
        &self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        label: &str,
        x: f32,
        y: f32,
        _w: f32,
    ) {
        ui.push_text(batch, label, x, y - 24.0, Color::new(0.6, 0.6, 0.65, 1.0));
    }
}
