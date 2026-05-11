use crate::graphics::DrawBatch;

pub struct RectRenderer;

impl Default for RectRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl RectRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, _batch: &mut DrawBatch, _screen_width: f32, _screen_height: f32) {}
}
