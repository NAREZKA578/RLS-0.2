use crate::font::font_atlas::FontAtlas;
use std::collections::HashMap;

pub struct FontCache {
    fonts: HashMap<String, FontAtlas>,
}

impl Default for FontCache {
    fn default() -> Self {
        Self::new()
    }
}

impl FontCache {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    pub fn load(&mut self, name: String, data: &[u8], _size: f32) -> Result<(), String> {
        let atlas = FontAtlas::from_bytes(data)?;
        self.fonts.insert(name, atlas);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&FontAtlas> {
        self.fonts.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut FontAtlas> {
        self.fonts.get_mut(name)
    }
}
