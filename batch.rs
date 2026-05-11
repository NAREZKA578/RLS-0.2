use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable, Debug)]
pub struct UiVertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
    pub mode: f32,
    pub radius: f32,
    pub half_size: [f32; 2],
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const GRAY: Color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    pub const DARK_GRAY: Color = Color {
        r: 0.2,
        g: 0.2,
        b: 0.2,
        a: 1.0,
    };
    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    pub const RTGC_ACCENT: Color = Color {
        r: 0.85,
        g: 0.55,
        b: 0.15,
        a: 1.0,
    };
    pub const RTGC_ACCENT2: Color = Color {
        r: 0.20,
        g: 0.55,
        b: 0.80,
        a: 1.0,
    };
    pub const RTGC_BTN_NORM: Color = Color {
        r: 0.12,
        g: 0.14,
        b: 0.16,
        a: 0.92,
    };
    pub const RTGC_BTN_HOVER: Color = Color {
        r: 0.20,
        g: 0.18,
        b: 0.15,
        a: 0.95,
    };
    pub const RTGC_BTN_PRESS: Color = Color {
        r: 0.85,
        g: 0.55,
        b: 0.15,
        a: 0.20,
    };
    pub const RTGC_TEXT: Color = Color {
        r: 0.92,
        g: 0.90,
        b: 0.86,
        a: 1.0,
    };
    pub const RTGC_TEXT_DIM: Color = Color {
        r: 0.52,
        g: 0.52,
        b: 0.50,
        a: 1.0,
    };
    pub const RTGC_BG: Color = Color {
        r: 0.05,
        g: 0.07,
        b: 0.09,
        a: 1.0,
    };
    pub const RTGC_BG_TOP: Color = Color {
        r: 0.04,
        g: 0.06,
        b: 0.10,
        a: 1.0,
    };
    pub const RTGC_BG_BOT: Color = Color {
        r: 0.06,
        g: 0.04,
        b: 0.04,
        a: 1.0,
    };
    pub const RTGC_CENTER_GLOW: Color = Color {
        r: 0.12,
        g: 0.10,
        b: 0.08,
        a: 0.15,
    };
    pub const RTGC_DIM_TEXT: Color = Color {
        r: 0.40,
        g: 0.40,
        b: 0.45,
        a: 1.0,
    };
    pub const RTGC_BTN_BG: Color = Color {
        r: 0.15,
        g: 0.14,
        b: 0.13,
        a: 0.9,
    };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }
}

pub struct DrawBatch {
    pub vertices: Vec<UiVertex>,
    vbo: Option<u32>,
    vao: Option<u32>,
    #[allow(dead_code)]
    current_tex: Option<u32>,
}

impl Default for DrawBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl DrawBatch {
    pub fn new() -> Self {
        Self {
            vertices: Vec::with_capacity(4096),
            vbo: None,
            vao: None,
            current_tex: None,
        }
    }

    pub fn push_rect(&mut self, rect: Rect, color: Color, corner_radius: f32) {
        let x0 = rect.x;
        let y0 = rect.y;
        let x1 = rect.x + rect.w;
        let y1 = rect.y + rect.h;
        let half_w = rect.w / 2.0;
        let half_h = rect.h / 2.0;

        let c = [color.r, color.g, color.b, color.a];

        let v = |px: f32, py: f32, u_unit: f32, v_unit: f32| UiVertex {
            pos: [px, py],
            uv: [u_unit, v_unit],
            color: c,
            mode: 0.0,
            radius: corner_radius,
            half_size: [half_w, half_h],
        };
        self.vertices.push(v(x0, y0, -1.0, -1.0));
        self.vertices.push(v(x1, y0, 1.0, -1.0));
        self.vertices.push(v(x1, y1, 1.0, 1.0));
        self.vertices.push(v(x0, y0, -1.0, -1.0));
        self.vertices.push(v(x1, y1, 1.0, 1.0));
        self.vertices.push(v(x0, y1, -1.0, 1.0));
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn set_gl_resources(&mut self, vbo: u32, vao: u32) {
        self.vbo = Some(vbo);
        self.vao = Some(vao);
    }

    pub fn push_text_glyphs(&mut self, glyphs: &[(f32, f32, f32, f32, f32, f32)], color: Color) {
        let c = [color.r, color.g, color.b, color.a];
        for (x, y, u0, v0, u1, v1) in glyphs {
            let w = u1 - u0;
            let h = v1 - v0;
            self.vertices.push(UiVertex {
                pos: [*x, *y],
                uv: [*u0, *v0],
                color: c,
                mode: 1.0,
                radius: 0.0,
                half_size: [0.0, 0.0],
            });
            self.vertices.push(UiVertex {
                pos: [*x + w, *y],
                uv: [*u1, *v0],
                color: c,
                mode: 1.0,
                radius: 0.0,
                half_size: [0.0, 0.0],
            });
            self.vertices.push(UiVertex {
                pos: [*x + w, *y + h],
                uv: [*u1, *v1],
                color: c,
                mode: 1.0,
                radius: 0.0,
                half_size: [0.0, 0.0],
            });
            self.vertices.push(UiVertex {
                pos: [*x, *y],
                uv: [*u0, *v0],
                color: c,
                mode: 1.0,
                radius: 0.0,
                half_size: [0.0, 0.0],
            });
            self.vertices.push(UiVertex {
                pos: [*x + w, *y + h],
                uv: [*u1, *v1],
                color: c,
                mode: 1.0,
                radius: 0.0,
                half_size: [0.0, 0.0],
            });
            self.vertices.push(UiVertex {
                pos: [*x, *y + h],
                uv: [*u0, *v1],
                color: c,
                mode: 1.0,
                radius: 0.0,
                half_size: [0.0, 0.0],
            });
        }
    }
}
