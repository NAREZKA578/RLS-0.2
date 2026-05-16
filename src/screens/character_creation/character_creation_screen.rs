use crate::core::app_state::{AppState, CharacterData, Gender};
use crate::core::character_creation_data::CharacterCreationData;
use crate::graphics::ui_renderer::batch::{Color, DrawBatch, Rect};
use crate::ui::panel::Panel;

const STEP_TITLES: [&str; 10] = [
    "ПОЛ",
    "РОСТ",
    "ЦВЕТ КОЖИ",
    "ЛИЦО",
    "ПРИЧЁСКА",
    "ЦВЕТ ВОЛОС",
    "УНИВЕРСИТЕТ",
    "СПЕЦИАЛЬНОСТЬ",
    "РЕГИОН",
    "ГОТОВО",
];

pub struct CharacterCreationScreen {
    current_step: u8,
    total_steps: u8,
    data: CharacterData,
    hovered: HoverTarget,
    dragging: bool,
    creation_data: CharacterCreationData,
}

#[derive(Clone, Copy, PartialEq)]
enum HoverTarget {
    None,
    Back,
    Next,
    Option(usize),
}

impl Default for CharacterCreationScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl CharacterCreationScreen {
    pub fn new() -> Self {
        let default_data = CharacterCreationData {
            universities: crate::core::character_creation_data::Universities {
                ngtu: crate::core::character_creation_data::University { name: "НГТУ".into(), code: "НГТУ".into() },
                ngu: crate::core::character_creation_data::University { name: "НГУ".into(), code: "НГУ".into() },
                ngmu: crate::core::character_creation_data::University { name: "НГМУ".into(), code: "НГМУ".into() },
                ngasu: crate::core::character_creation_data::University { name: "НГАСУ".into(), code: "НГАСУ".into() },
                ngpu: crate::core::character_creation_data::University { name: "НГПУ".into(), code: "НГПУ".into() },
                sibguti: crate::core::character_creation_data::University { name: "СибГУТИ".into(), code: "СибГУТИ".into() },
            },
            specialties: crate::core::character_creation_data::Specialties {
                engineering: "Инженерия".into(),
                mechanics: "Механика".into(),
                economics: "Экономика".into(),
                law: "Право".into(),
            },
            capital_table: crate::core::character_creation_data::CapitalTable {
                ngtu: crate::core::character_creation_data::SpecialtyRow { engineering: 40000.0, mechanics: 55000.0, economics: 70000.0, law: 90000.0 },
                ngu: crate::core::character_creation_data::SpecialtyRow { engineering: 35000.0, mechanics: 50000.0, economics: 65000.0, law: 80000.0 },
                ngmu: crate::core::character_creation_data::SpecialtyRow { engineering: 45000.0, mechanics: 60000.0, economics: 80000.0, law: 100000.0 },
                ngasu: crate::core::character_creation_data::SpecialtyRow { engineering: 30000.0, mechanics: 45000.0, economics: 60000.0, law: 75000.0 },
                ngpu: crate::core::character_creation_data::SpecialtyRow { engineering: 32000.0, mechanics: 48000.0, economics: 62000.0, law: 82000.0 },
                sibguti: crate::core::character_creation_data::SpecialtyRow { engineering: 38000.0, mechanics: 52000.0, economics: 68000.0, law: 88000.0 },
            },
            regions: vec![
                crate::core::character_creation_data::Region { name: "Центр".into(), position: [0.0, 0.0, 0.0], color: [0.85, 0.55, 0.15] },
                crate::core::character_creation_data::Region { name: "Ленинский".into(), position: [3000.0, 0.0, 4000.0], color: [0.2, 0.6, 0.85] },
                crate::core::character_creation_data::Region { name: "Первомайский".into(), position: [-5000.0, 0.0, 3000.0], color: [0.3, 0.7, 0.4] },
                crate::core::character_creation_data::Region { name: "Кировский".into(), position: [4000.0, 0.0, -5000.0], color: [0.85, 0.3, 0.3] },
                crate::core::character_creation_data::Region { name: "Советский".into(), position: [0.0, 0.0, -8000.0], color: [0.6, 0.3, 0.8] },
                crate::core::character_creation_data::Region { name: "Дзержинский".into(), position: [-6000.0, 0.0, -2000.0], color: [0.85, 0.65, 0.2] },
                crate::core::character_creation_data::Region { name: "Залив".into(), position: [6000.0, 0.0, 10000.0], color: [0.2, 0.5, 0.7] },
                crate::core::character_creation_data::Region { name: "Нарымский".into(), position: [-3000.0, 0.0, -7000.0], color: [0.5, 0.5, 0.5] },
                crate::core::character_creation_data::Region { name: "Маршала Пожарова".into(), position: [8000.0, 0.0, -3000.0], color: [0.7, 0.4, 0.6] },
                crate::core::character_creation_data::Region { name: "Старый Сибирь".into(), position: [-7000.0, 0.0, 5000.0], color: [0.4, 0.65, 0.3] },
                crate::core::character_creation_data::Region { name: "Октябрьский".into(), position: [-2000.0, 0.0, 6000.0], color: [0.3, 0.5, 0.8] },
                crate::core::character_creation_data::Region { name: "Калининский".into(), position: [5000.0, 0.0, 3000.0], color: [0.8, 0.5, 0.3] },
            ],
            skin_colors: crate::core::character_creation_data::ColorList {
                colors: vec![
                    crate::core::character_creation_data::ColorEntry { name: "Светлый".into(), rgb: [0.98, 0.87, 0.78] },
                    crate::core::character_creation_data::ColorEntry { name: "Средний".into(), rgb: [0.91, 0.73, 0.60] },
                    crate::core::character_creation_data::ColorEntry { name: "Смуглый".into(), rgb: [0.74, 0.52, 0.38] },
                    crate::core::character_creation_data::ColorEntry { name: "Тёмный".into(), rgb: [0.50, 0.33, 0.22] },
                    crate::core::character_creation_data::ColorEntry { name: "Очень тёмный".into(), rgb: [0.27, 0.17, 0.11] },
                ],
            },
            hair_colors: crate::core::character_creation_data::ColorList {
                colors: vec![
                    crate::core::character_creation_data::ColorEntry { name: "Чёрные".into(), rgb: [0.10, 0.06, 0.04] },
                    crate::core::character_creation_data::ColorEntry { name: "Каштановые".into(), rgb: [0.35, 0.20, 0.10] },
                    crate::core::character_creation_data::ColorEntry { name: "Русые".into(), rgb: [0.65, 0.42, 0.22] },
                    crate::core::character_creation_data::ColorEntry { name: "Блонд".into(), rgb: [0.88, 0.74, 0.40] },
                    crate::core::character_creation_data::ColorEntry { name: "Рыжие".into(), rgb: [0.75, 0.30, 0.08] },
                ],
            },
        };

        let creation_data = CharacterCreationData::load(std::path::Path::new("config/character_creation_data.toml"))
            .unwrap_or_else(|| {
                tracing::info!("Using default character creation data");
                default_data
            });

        Self {
            current_step: 1,
            total_steps: 10,
            data: CharacterData::default(),
            hovered: HoverTarget::None,
            dragging: false,
            creation_data,
        }
    }

    pub fn current_step(&self) -> u8 {
        self.current_step
    }

    fn panel(&self, sw: f32, sh: f32) -> Rect {
        let s = (self.current_step - 1) as usize;
        let (w, h) = if s == 8 {
            (900.0, 500.0)
        } else {
            (700.0, 420.0)
        };
        Rect {
            x: (sw - w) / 2.0,
            y: (sh - h) / 2.0 - 10.0,
            w,
            h,
        }
    }

    fn preview_rect(&self, sw: f32, sh: f32) -> Rect {
        let p = self.panel(sw, sh);
        Rect {
            x: p.x + 10.0,
            y: p.y + 55.0,
            w: 170.0,
            h: p.h - 75.0,
        }
    }

    fn options_area(&self, sw: f32, sh: f32) -> Rect {
        let p = self.panel(sw, sh);
        let pv = self.preview_rect(sw, sh);
        let s = (self.current_step - 1) as usize;
        
        let options_width = if s == 8 {
            p.w - pv.w - 25.0
        } else {
            p.w - pv.w - 25.0
        };
        
        Rect {
            x: pv.x + pv.w + 15.0,
            y: pv.y,
            w: options_width,
            h: pv.h,
        }
    }

    fn btn_back(&self, sw: f32, sh: f32) -> Rect {
        let p = self.panel(sw, sh);
        Rect {
            x: p.x + 15.0,
            y: p.y + p.h + 12.0,
            w: 140.0,
            h: 42.0,
        }
    }

    fn btn_next(&self, sw: f32, sh: f32) -> Rect {
        let p = self.panel(sw, sh);
        Rect {
            x: p.x + p.w - 155.0,
            y: p.y + p.h + 12.0,
            w: 140.0,
            h: 42.0,
        }
    }

    fn selected_idx(&self) -> usize {
        let s = (self.current_step - 1) as usize;
        match s {
            0 => {
                if self.data.gender == Gender::Male {
                    0
                } else {
                    1
                }
            }
            2 => self.data.skin_color as usize,
            5 => {
                let hc = self.data.hair_color;
                let hair_colors = self.creation_data.get_hair_colors();
                hair_colors
                    .iter()
                    .position(|c| c == &[hc[0], hc[1], hc[2]])
                    .unwrap_or(0)
            }
            6 => {
                let universities = self.creation_data.get_universities_list();
                universities
                    .iter()
                    .position(|u| *u == self.data.university_id)
                    .unwrap_or(0)
            }
            7 => {
                let specialties = self.creation_data.get_specialties_list();
                specialties
                    .iter()
                    .position(|s| *s == self.data.specialty)
                    .unwrap_or(0)
            }
            8 => {
                let regions = self.creation_data.get_regions();
                regions
                    .iter()
                    .position(|r| r.name == self.data.start_region)
                    .unwrap_or(0)
            }
            _ => 0,
        }
    }

    fn update_capital(&mut self) {
        self.data.start_capital = self.creation_data.get_capital(&self.data.university_id, &self.data.specialty);
    }

    fn get_skin_colors(&self) -> Vec<[f32; 3]> {
        self.creation_data.get_skin_colors()
    }

    fn get_hair_colors(&self) -> Vec<[f32; 3]> {
        self.creation_data.get_hair_colors()
    }

    fn get_universities(&self) -> Vec<&str> {
        self.creation_data.get_universities_list()
    }

    fn get_specialties(&self) -> Vec<&str> {
        self.creation_data.get_specialties_list()
    }

    fn get_regions(&self) -> Vec<&crate::core::character_creation_data::Region> {
        self.creation_data.get_regions().iter().collect()
    }

    fn update_hover(&mut self, mx: f32, my: f32, sw: f32, sh: f32) {
        if self.btn_back(sw, sh).contains(mx, my) {
            self.hovered = HoverTarget::Back;
            return;
        }
        if self.btn_next(sw, sh).contains(mx, my) {
            self.hovered = HoverTarget::Next;
            return;
        }

        let s = (self.current_step - 1) as usize;
        match s {
            0 => {
                let a = self.options_area(sw, sh);
                let bw = a.w * 0.45;
                let bh = 48.0;
                let gap = a.w - bw * 2.0;
                for i in 0..2 {
                    let r = Rect {
                        x: a.x + i as f32 * (bw + gap),
                        y: a.y + a.h / 2.0 - bh / 2.0,
                        w: bw,
                        h: bh,
                    };
                    if r.contains(mx, my) {
                        self.hovered = HoverTarget::Option(i);
                        return;
                    }
                }
            }
            2 | 5 => {
                let a = self.options_area(sw, sh);
                let bw = 64.0;
                let gap = 12.0;
                let total = 5.0 * bw + 4.0 * gap;
                let sx = a.x + (a.w - total) / 2.0;
                let cy = a.y + a.h / 2.0 - bw / 2.0 - 16.0;
                for i in 0..5 {
                    let r = Rect {
                        x: sx + i as f32 * (bw + gap),
                        y: cy,
                        w: bw,
                        h: bw,
                    };
                    if r.contains(mx, my) {
                        self.hovered = HoverTarget::Option(i);
                        return;
                    }
                }
            }
            1 => {
                let a = self.options_area(sw, sh);
                let slider_w = a.w * 0.8;
                let sx = a.x + (a.w - slider_w) / 2.0;
                let sy = a.y + a.h / 2.0;
                if mx >= sx - 10.0
                    && mx <= sx + slider_w + 10.0
                    && my >= sy - 20.0
                    && my <= sy + 20.0
                {
                    self.hovered = HoverTarget::Option(0);
                    return;
                }
            }
            8 => {
                let a = self.options_area(sw, sh);
                let cols = 3;
                let min_bw = 120.0;
                let bw = (a.w / cols as f32).min(min_bw * 1.3).max(min_bw);
                let bh = 38.0;
                let total = cols as f32 * bw;
                let gap = (a.w - total) / (cols as f32 + 1.0);
                let sx = a.x + gap;
                let by = a.y + a.h - bh - 10.0;
                for i in 0..12 {
                    let col = i % cols;
                    let row = i / cols;
                    let r = Rect {
                        x: sx + col as f32 * (bw + gap),
                        y: by - row as f32 * (bh + 8.0),
                        w: bw,
                        h: bh,
                    };
                    if r.contains(mx, my) {
                        self.hovered = HoverTarget::Option(i);
                        return;
                    }
                }
            }
            9 => {
                let a = self.options_area(sw, sh);
                let bw = a.w * 0.22;
                let bh = 44.0;
                let gap = (a.w - 4.0 * bw) / 3.0;
                let cy = a.y + a.h / 2.0 - bh / 2.0 + 50.0;
                for i in 0..4 {
                    let r = Rect {
                        x: a.x + i as f32 * (bw + gap),
                        y: cy,
                        w: bw,
                        h: bh,
                    };
                    if r.contains(mx, my) {
                        self.hovered = HoverTarget::Option(i);
                        return;
                    }
                }
            }
            _ => {}
        }
        self.hovered = HoverTarget::None;
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
        self.update_hover(mouse_x, mouse_y, screen_w, screen_h);

        let s = (self.current_step - 1) as usize;
        if s == 1 && (self.dragging || mouse_held) {
            let a = self.options_area(screen_w, screen_h);
            let slider_w = a.w * 0.8;
            let sx = a.x + (a.w - slider_w) / 2.0;
            if mouse_x >= sx - 10.0 && mouse_x <= sx + slider_w + 10.0 {
                let t = ((mouse_x - sx) / slider_w).clamp(0.0, 1.0);
                self.data.height_m = 1.50 + t * 0.50;
                self.dragging = true;
            }
        }
        if !mouse_held {
            self.dragging = false;
        }

        if mouse_just_pressed {
            if self.btn_back(screen_w, screen_h).contains(mouse_x, mouse_y) {
                if self.current_step > 1 {
                    self.current_step -= 1;
                } else {
                    return Some(AppState::MainMenu);
                }
                return None;
            }
            if self.btn_next(screen_w, screen_h).contains(mouse_x, mouse_y) {
                if s == 9 {
                    return Some(AppState::Loading {
                        character_data: Box::new(self.data.clone()),
                    });
                }
                if self.current_step < self.total_steps {
                    self.current_step += 1;
                }
                return None;
            }

            match s {
                0 => {
                    let a = self.options_area(screen_w, screen_h);
                    let bw = a.w * 0.45;
                    let bh = 48.0;
                    let gap = a.w - bw * 2.0;
                    for i in 0..2 {
                        let r = Rect {
                            x: a.x + i as f32 * (bw + gap),
                            y: a.y + a.h / 2.0 - bh / 2.0,
                            w: bw,
                            h: bh,
                        };
                        if r.contains(mouse_x, mouse_y) {
                            self.data.gender = if i == 0 { Gender::Male } else { Gender::Female };
                        }
                    }
                }
                2 => {
                    let a = self.options_area(screen_w, screen_h);
                    let bw = 68.0;
                    let gap = 10.0;
                    let total = 5.0 * bw + 4.0 * gap;
                    let sx = a.x + (a.w - total) / 2.0;
                    let cy = a.y + a.h / 2.0 - bw / 2.0 - 16.0;
                    for i in 0..5 {
                        let r = Rect {
                            x: sx + i as f32 * (bw + gap),
                            y: cy,
                            w: bw,
                            h: bw,
                        };
                        if r.contains(mouse_x, mouse_y) {
                            self.data.skin_color = i as u8;
                        }
                    }
                }
                5 => {
                    let a = self.options_area(screen_w, screen_h);
                    let bw = 68.0;
                    let gap = 10.0;
                    let total = 5.0 * bw + 4.0 * gap;
                    let sx = a.x + (a.w - total) / 2.0;
                    let cy = a.y + a.h / 2.0 - bw / 2.0 - 16.0;
                    for (i, &color) in self.get_hair_colors().iter().enumerate().take(5) {
                        let r = Rect {
                            x: sx + i as f32 * (bw + gap),
                            y: cy,
                            w: bw,
                            h: bw,
                        };
                        if r.contains(mouse_x, mouse_y) {
                            self.data.hair_color = color;
                        }
                    }
                }
6 => {
                    let universities: Vec<&str> = self.creation_data.get_universities_list();
                    let a = self.options_area(screen_w, screen_h);
                    let cols = 6;
                    let min_bw = 80.0;
                    let bw = (a.w / cols as f32).min(min_bw * 1.5).max(min_bw);
                    let bh = 44.0;
                    let total = cols as f32 * bw;
                    let gap = (a.w - total) / (cols as f32 + 1.0);
                    let sx = a.x + gap;
                    let cy = a.y + a.h / 2.0 - bh / 2.0;
                    let mut selected_uni: Option<&str> = None;
                    for (i, uni) in universities.iter().enumerate() {
                        let r = Rect {
                            x: sx + i as f32 * (bw + gap),
                            y: cy,
                            w: bw,
                            h: bh,
                        };
                        if r.contains(mouse_x, mouse_y) {
                            selected_uni = Some(uni);
                        }
                    }
                    if let Some(uni) = selected_uni {
                        self.data.university_id = uni.to_string();
                        self.update_capital();
                    }
                }
                7 => {
                    let specialties: Vec<&str> = self.creation_data.get_specialties_list();
                    let a = self.options_area(screen_w, screen_h);
                    let cols = 4;
                    let min_bw = 80.0;
                    let bw = (a.w / cols as f32).min(min_bw * 1.5).max(min_bw);
                    let bh = 44.0;
                    let total = cols as f32 * bw;
                    let gap = (a.w - total) / (cols as f32 + 1.0);
                    let sx = a.x + gap;
                    let cy = a.y + a.h / 2.0 - bh / 2.0;
                    let mut selected_spec: Option<&str> = None;
                    for (i, spec) in specialties.iter().enumerate().take(4) {
                        let r = Rect {
                            x: sx + i as f32 * (bw + gap),
                            y: cy,
                            w: bw,
                            h: bh,
                        };
                        if r.contains(mouse_x, mouse_y) {
                            selected_spec = Some(spec);
                        }
                    }
                    if let Some(spec) = selected_spec {
                        self.data.specialty = spec.to_string();
                        self.update_capital();
                    }
                }
                8 => {
                    let a = self.options_area(screen_w, screen_h);
                    let map_h = a.h * 0.55;
                    let my = a.y + 10.0;
                    let regions = self.creation_data.get_regions().to_vec();
                    let by = my + map_h + 15.0;
                    let cols = 4;
                    let min_bw = 80.0;
                    let bw = (a.w / cols as f32).min(min_bw * 1.5).max(min_bw);
                    let total = cols as f32 * bw;
                    let gap = (a.w - total) / (cols as f32 + 1.0);
                    let sx = a.x + gap;
                    let bh = 40.0;
                    for i in 0..4.min(regions.len()) {
                        let r = Rect {
                            x: sx + i as f32 * (bw + gap),
                            y: by,
                            w: bw,
                            h: bh,
                        };
                        if r.contains(mouse_x, mouse_y) {
                            self.data.start_region = regions[i].name.clone();
                            self.data.start_pos = regions[i].position;
                        }
                    }
                }
                _ => {}
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
            Color::new(0.06, 0.06, 0.1, 1.0),
            0.0,
        );

        let p = self.panel(sw, sh);
        Panel::new(p.x, p.y, p.w, p.h, Color::new(0.12, 0.12, 0.16, 0.95))
            .with_corner_radius(8.0)
            .render(batch);

        let title = STEP_TITLES[(self.current_step - 1) as usize];
        let tw = ui.measure_text_width(title);
        ui.push_text(
            batch,
            title,
            p.x + (p.w - tw) / 2.0,
            p.y + 18.0,
            Color::WHITE,
        );

        let st = format!("Шаг {} / {}", self.current_step, self.total_steps);
        let sw2 = ui.measure_text_width(&st);
        ui.push_text(
            batch,
            &st,
            p.x + (p.w - sw2) / 2.0,
            p.y + 42.0,
            Color::new(0.45, 0.45, 0.5, 1.0),
        );

        self.render_step_indicators(batch, ui, p.x + p.w / 2.0, p.y + 68.0, self.total_steps);

        let pv = self.preview_rect(sw, sh);
        Panel::new(pv.x, pv.y, pv.w, pv.h, Color::new(0.08, 0.08, 0.12, 1.0))
            .with_corner_radius(4.0)
            .render(batch);
        self.render_preview(batch, ui, pv);

        let a = self.options_area(sw, sh);
        let s = (self.current_step - 1) as usize;
        let skin_labels = ["Свет.", "Ср.", "Смугл.", "Тёмн.", "Оч.тёмн."];
        let hair_labels = ["Чёрн.", "Кашт.", "Русые", "Блонд", "Рыжие"];
        let skin_colors = self.get_skin_colors();
        let hair_colors = self.get_hair_colors();
        match s {
            0 => self.render_gender(batch, ui, a),
            1 => self.render_height(batch, ui, a),
            2 => self.render_color_swatches(
                batch,
                ui,
                a,
                &skin_colors,
                &skin_labels,
            ),
            3 => self.render_placeholder(batch, ui, a, "Лицо", "Будет доступно в полной версии"),
            4 => {
                self.render_placeholder(batch, ui, a, "Причёска", "Будет доступно в полной версии")
            }
            5 => self.render_color_swatches(
                batch,
                ui,
                a,
                &hair_colors,
                &hair_labels,
            ),
            6 => self.render_options(batch, ui, a, &self.get_universities()),
            7 => self.render_options(batch, ui, a, &self.get_specialties()),
            8 => self.render_region(batch, ui, a),
            9 => self.render_summary(batch, ui, a),
            _ => {}
        }

        let back_c = if matches!(self.hovered, HoverTarget::Back) {
            Color::new(0.28, 0.28, 0.34, 1.0)
        } else {
            Color::new(0.18, 0.18, 0.22, 0.9)
        };
        let next_c = if matches!(self.hovered, HoverTarget::Next) {
            Color::new(0.28, 0.28, 0.34, 1.0)
        } else {
            Color::new(0.18, 0.18, 0.22, 0.9)
        };
        batch.push_rect(self.btn_back(sw, sh), back_c, 4.0);
        batch.push_rect(self.btn_next(sw, sh), next_c, 4.0);

        let bt = "НАЗАД";
        let nt = if s == 9 { "НАЧАТЬ" } else { "ДАЛЕЕ" };
        let bw = ui.measure_text_width(bt);
        let nw = ui.measure_text_width(nt);
        let br = self.btn_back(sw, sh);
        let nr = self.btn_next(sw, sh);
        ui.push_text(
            batch,
            bt,
            br.x + (br.w - bw) / 2.0,
            br.y + (br.h - 20.0) / 2.0 + 6.0,
            Color::WHITE,
        );
        ui.push_text(
            batch,
            nt,
            nr.x + (nr.w - nw) / 2.0,
            nr.y + (nr.h - 20.0) / 2.0 + 6.0,
            Color::WHITE,
        );
    }

    fn render_preview(&self, batch: &mut DrawBatch, ui: &crate::graphics::UiRenderer, r: Rect) {
        let cy = r.y + r.h * 0.4;
        let label = if self.data.gender == Gender::Male {
            "МУЖ"
        } else {
            "ЖЕН"
        };
        let lw = ui.measure_text_width(label);
        ui.push_text(
            batch,
            label,
            r.x + (r.w - lw) / 2.0,
            cy - 10.0,
            Color::new(0.8, 0.8, 0.85, 1.0),
        );

        let skin_colors = &self.get_skin_colors();
        let skin = skin_colors[(self.data.skin_color as usize).min(skin_colors.len() - 1)];
        let head_r = 20.0;
        batch.push_rect(
            Rect {
                x: r.x + r.w / 2.0 - head_r,
                y: cy - 40.0,
                w: head_r * 2.0,
                h: head_r * 2.0,
            },
            Color::new(skin[0], skin[1], skin[2], 1.0),
            10.0,
        );
        batch.push_rect(
            Rect {
                x: r.x + r.w / 2.0 - 12.0,
                y: cy - 15.0,
                w: 24.0,
                h: 40.0,
            },
            Color::new(skin[0], skin[1], skin[2], 1.0),
            4.0,
        );

        let hair = self.data.hair_color;
        batch.push_rect(
            Rect {
                x: r.x + r.w / 2.0 - head_r,
                y: cy - 45.0,
                w: head_r * 2.0,
                h: 10.0,
            },
            Color::new(hair[0], hair[1], hair[2], 1.0),
            4.0,
        );

        ui.push_text(
            batch,
            "Одежда",
            r.x + 10.0,
            r.y + r.h - 25.0,
            Color::new(0.4, 0.4, 0.45, 1.0),
        );
    }

    fn render_gender(&self, batch: &mut DrawBatch, ui: &crate::graphics::UiRenderer, a: Rect) {
        let bw = a.w * 0.45;
        let bh = 48.0;
        let gap = a.w - bw * 2.0;
        let cy = a.y + a.h / 2.0 - bh / 2.0;
        for i in 0..2 {
            let r = Rect {
                x: a.x + i as f32 * (bw + gap),
                y: cy,
                w: bw,
                h: bh,
            };
            let sel = self.selected_idx() == i;
            let hov = matches!(self.hovered, HoverTarget::Option(j) if j == i);
            let bg = if sel {
                Color::new(0.85, 0.55, 0.15, 0.85)
            } else if hov {
                Color::new(0.24, 0.24, 0.3, 1.0)
            } else {
                Color::new(0.18, 0.18, 0.22, 0.9)
            };
            batch.push_rect(r, bg, 6.0);
            let label = if i == 0 {
                "МУЖСКОЙ"
            } else {
                "ЖЕНСКИЙ"
            };
            let lw = ui.measure_text_width(label);
            ui.push_text(
                batch,
                label,
                r.x + (r.w - lw) / 2.0,
                r.y + (r.h - 20.0) / 2.0 + 6.0,
                Color::WHITE,
            );
        }
    }

    fn render_height(&self, batch: &mut DrawBatch, ui: &crate::graphics::UiRenderer, a: Rect) {
        let slider_w = a.w * 0.8;
        let sx = a.x + (a.w - slider_w) / 2.0;
        let sy = a.y + a.h / 2.0 - 5.0;
        let sht = 8.0;

        batch.push_rect(
            Rect {
                x: sx,
                y: sy,
                w: slider_w,
                h: sht,
            },
            Color::new(0.2, 0.2, 0.25, 1.0),
            4.0,
        );

        let t = (self.data.height_m - 1.50) / 0.50;
        let fill_w = slider_w * t;
        batch.push_rect(
            Rect {
                x: sx,
                y: sy,
                w: fill_w,
                h: sht,
            },
            Color::new(0.85, 0.55, 0.15, 0.9),
            4.0,
        );

        let knob_x = sx + fill_w - 8.0;
        batch.push_rect(
            Rect {
                x: knob_x,
                y: sy - 6.0,
                w: 16.0,
                h: sht + 12.0,
            },
            Color::new(0.85, 0.85, 0.9, 1.0),
            8.0,
        );

        let label = format!("{:.2} м", self.data.height_m);
        let lw = ui.measure_text_width(&label);
        ui.push_text(
            batch,
            &label,
            a.x + (a.w - lw) / 2.0,
            sy - 30.0,
            Color::WHITE,
        );

        let min_lw = ui.measure_text_width("1.50");
        let max_lw = ui.measure_text_width("2.00");
        ui.push_text(
            batch,
            "1.50",
            sx - min_lw / 2.0,
            sy + 14.0,
            Color::new(0.4, 0.4, 0.45, 1.0),
        );
        ui.push_text(
            batch,
            "2.00",
            sx + slider_w - max_lw / 2.0,
            sy + 14.0,
            Color::new(0.4, 0.4, 0.45, 1.0),
        );
    }

    fn render_color_swatches(
        &self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        a: Rect,
        colors: &[[f32; 3]],
        labels: &[&str],
    ) {
        let bw = 68.0;
        let gap = 10.0;
        let total = 5.0 * bw + 4.0 * gap;
        let sx = a.x + (a.w - total) / 2.0;
        let cy = a.y + a.h / 2.0 - bw / 2.0 - 14.0;
        for i in 0..5.min(colors.len()).min(labels.len()) {
            let col = colors[i];
            let r = Rect {
                x: sx + i as f32 * (bw + gap),
                y: cy,
                w: bw,
                h: bw,
            };
            let sel = self.selected_idx() == i;
            batch.push_rect(r, Color::new(col[0], col[1], col[2], 1.0), 6.0);
            if sel {
                batch.push_rect(
                    Rect {
                        x: r.x - 3.0,
                        y: r.y - 3.0,
                        w: r.w + 6.0,
                        h: r.h + 6.0,
                    },
                    Color::new(0.9, 0.75, 0.15, 1.0),
                    8.0,
                );
            }
            let lw = ui.measure_text_width(labels[i]);
            ui.push_text(
                batch,
                labels[i],
                r.x + (r.w - lw) / 2.0,
                r.y + r.h + 8.0,
                Color::new(0.5, 0.5, 0.55, 1.0),
            );
        }
    }

    fn render_step_indicators(
        &self,
        batch: &mut DrawBatch,
        _ui: &crate::graphics::UiRenderer,
        cx: f32,
        y: f32,
        total: u8,
    ) {
        let dot_r = 5.0;
        let spacing = 22.0;
        let total_w = total as f32 * spacing;
        let start_x = cx - total_w / 2.0;

        batch.push_rect(
            Rect {
                x: start_x + dot_r,
                y: y - 1.0,
                w: total_w - dot_r,
                h: 2.0,
            },
            Color::new(0.20, 0.20, 0.20, 1.0),
            0.0,
        );

        for i in 0..total {
            let x = start_x + i as f32 * spacing + dot_r;
            let color = if i < self.current_step {
                Color::new(0.85, 0.55, 0.15, 1.0)
            } else if i == self.current_step - 1 {
                Color::new(0.92, 0.90, 0.86, 1.0)
            } else {
                Color::new(0.25, 0.25, 0.25, 1.0)
            };
            batch.push_rect(
                Rect {
                    x: x - dot_r,
                    y: y - dot_r,
                    w: dot_r * 2.0,
                    h: dot_r * 2.0,
                },
                color,
                dot_r,
            );
        }
    }

    fn render_placeholder(
        &self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        a: Rect,
        title: &str,
        hint: &str,
    ) {
        let tw = ui.measure_text_width(title);
        ui.push_text(
            batch,
            title,
            a.x + (a.w - tw) / 2.0,
            a.y + a.h / 2.0 - 15.0,
            Color::new(0.6, 0.6, 0.65, 1.0),
        );
        let hw = ui.measure_text_width(hint);
        ui.push_text(
            batch,
            hint,
            a.x + (a.w - hw) / 2.0,
            a.y + a.h / 2.0 + 15.0,
            Color::new(0.35, 0.35, 0.4, 1.0),
        );
    }

    fn render_options(
        &self,
        batch: &mut DrawBatch,
        ui: &crate::graphics::UiRenderer,
        a: Rect,
        options: &[&str],
    ) {
        let cols = options.len();
        let min_bw = 80.0;
        let bw = (a.w / cols as f32).min(min_bw * 1.5).max(min_bw);
        let bh = 44.0;
        let total = cols as f32 * bw;
        let gap = (a.w - total) / (cols as f32 + 1.0);
        let sx = a.x + gap;
        let cy = a.y + a.h / 2.0 - bh / 2.0;
        for (i, &opt) in options.iter().enumerate() {
            let r = Rect {
                x: sx + i as f32 * (bw + gap),
                y: cy,
                w: bw,
                h: bh,
            };
            let sel = self.selected_idx() == i;
            let hov = matches!(self.hovered, HoverTarget::Option(j) if j == i);
            let bg = if sel {
                Color::new(0.85, 0.55, 0.15, 0.85)
            } else if hov {
                Color::new(0.24, 0.24, 0.3, 1.0)
            } else {
                Color::new(0.18, 0.18, 0.22, 0.9)
            };
            batch.push_rect(r, bg, 6.0);
            let lw = ui.measure_text_width(opt);
            ui.push_text(
                batch,
                opt,
                r.x + (r.w - lw) / 2.0,
                r.y + (r.h - 20.0) / 2.0 + 6.0,
                Color::WHITE,
            );
        }
    }

    fn render_region(&self, batch: &mut DrawBatch, ui: &crate::graphics::UiRenderer, a: Rect) {
        let map_h = a.h * 0.55;
        let my = a.y + 10.0;
        Panel::new(a.x, my, a.w, map_h, Color::new(0.1, 0.12, 0.15, 1.0))
            .with_corner_radius(4.0)
            .render(batch);

        let cx = a.x + a.w / 2.0;
        let cy = a.y + a.h / 2.0 - 30.0;

        let region_screen_positions: [(f32, f32); 12] = [
            (cx, cy),                        // Центр
            (cx + 50.0, cy + 40.0),         // Ленинский
            (cx - 80.0, cy + 60.0),         // Первомайский
            (cx + 60.0, cy - 70.0),         // Кировский
            (cx + 10.0, cy - 100.0),        // Советский
            (cx - 70.0, cy - 20.0),         // Дзержинский
            (cx + 100.0, cy + 100.0),       // Залив
            (cx - 40.0, cy - 80.0),         // Нарымский
            (cx + 90.0, cy - 30.0),         // Маршала Пожарова
            (cx - 110.0, cy + 70.0),        // Старый Сибирь
            (cx + 30.0, cy + 80.0),         // Октябрьский
            (cx + 70.0, cy + 10.0),          // Калининский
        ];

        let sel_idx = self.selected_idx();
        let regions: Vec<_> = self.creation_data.get_regions().to_vec();

        for i in 0..12.min(regions.len()) {
            let (dx, dy) = region_screen_positions[i];
            let is_sel = sel_idx == i;
            let r = if is_sel { 12.0 } else { 9.0 };

            let region_color = regions[i].color;
            let dot_color = if is_sel {
                Color::new(1.0, 0.9, 0.3, 1.0)
            } else {
                Color::new(region_color[0], region_color[1], region_color[2], 1.0)
            };

            batch.push_rect(
                Rect {
                    x: dx - r,
                    y: dy - r,
                    w: r * 2.0,
                    h: r * 2.0,
                },
                dot_color,
                r,
            );

            if is_sel {
                batch.push_rect(
                    Rect {
                        x: dx - r - 4.0,
                        y: dy - r - 4.0,
                        w: r * 2.0 + 8.0,
                        h: r * 2.0 + 8.0,
                    },
                    Color::new(1.0, 0.9, 0.3, 0.5),
                    r + 4.0,
                );
            }

            let name = &regions[i].name;
            let lw = ui.measure_text_width(name);
            ui.push_text(
                batch,
                name,
                dx - lw / 2.0,
                dy + r + 8.0,
                if is_sel {
                    Color::new(1.0, 0.9, 0.3, 1.0)
                } else {
                    Color::new(0.7, 0.7, 0.75, 1.0)
                },
            );
        }
        
        let label = "КАРТА НОВОСИБИРСКА";
        let lw = ui.measure_text_width(label);
        ui.push_text(
            batch,
            label,
            cx - lw / 2.0,
            my + 15.0,
            Color::new(0.85, 0.55, 0.15, 1.0),
        );
    }

    fn render_summary(&self, batch: &mut DrawBatch, ui: &crate::graphics::UiRenderer, a: Rect) {
        let lines = [
            format!(
                "Пол: {}",
                if self.data.gender == Gender::Male {
                    "Мужской"
                } else {
                    "Женский"
                }
            ),
            format!("Рост: {:.2} м", self.data.height_m),
            format!(
                "Вуз: {}",
                if self.data.university_id.is_empty() {
                    "—"
                } else {
                    &self.data.university_id
                }
            ),
            format!(
                "Специальность: {}",
                if self.data.specialty.is_empty() {
                    "—"
                } else {
                    &self.data.specialty
                }
            ),
            format!("Стартовый капитал: {:.0} ₽", self.data.start_capital),
            format!(
                "Регион: {}",
                if self.data.start_region.is_empty() {
                    "—"
                } else {
                    &self.data.start_region
                }
            ),
        ];
        let sy = a.y + 20.0;
        for (i, line) in lines.iter().enumerate() {
            let lw = ui.measure_text_width(line);
            ui.push_text(
                batch,
                line,
                a.x + (a.w - lw) / 2.0,
                sy + i as f32 * 30.0,
                Color::new(0.7, 0.7, 0.75, 1.0),
            );
        }
    }
}
