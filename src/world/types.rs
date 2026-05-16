use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub name: String,
    pub seed: u64,
    pub layers: Vec<Layer>,
    pub cities: Vec<City>,
    pub objects: Vec<WorldObject>,
}

fn deserialize_null_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| s != "null"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub height: f32,
    pub tile_size: u32,
    pub chunks_x: u32,
    pub chunks_z: u32,
    pub visible: bool,
    pub locked: bool,
}

impl Default for Layer {
    fn default() -> Self {
        Self {
            name: String::new(),
            height: 0.0,
            tile_size: 32,
            chunks_x: 8,
            chunks_z: 8,
            visible: true,
            locked: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub id: String,
    pub name: String,
    pub position: [f32; 3],
    pub size: f32,
    pub layer_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldObject {
    pub id: String,
    pub name: String,
    pub object_type: String,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub layer_index: usize,
    #[serde(deserialize_with = "deserialize_null_string")]
    pub city_id: Option<String>,
    pub static_mesh: Option<String>,
}

impl Default for WorldObject {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            object_type: String::new(),
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            layer_index: 0,
            city_id: None,
            static_mesh: None,
        }
    }
}

impl World {
    pub fn get_layer(&self, index: usize) -> Option<&Layer> {
        self.layers.get(index)
    }

    pub fn ground_layer(&self) -> Option<&Layer> {
        self.layers.iter().find(|l| l.name.to_lowercase().contains("земля") || l.name.to_lowercase() == "ground")
    }

    pub fn ground_params(&self) -> (u32, u32, f32) {
        let layer = self.ground_layer();
        match layer {
            Some(l) => (l.tile_size, l.chunks_x * l.chunks_z, l.height),
            None => (50, 300 * 600, 0.0),
        }
    }

    pub fn get_city(&self, id: &str) -> Option<&City> {
        self.cities.iter().find(|c| c.id == id)
    }

    pub fn get_objects_in_city(&self, city_id: &str) -> Vec<&WorldObject> {
        self.objects.iter().filter(|o| o.city_id.as_deref() == Some(city_id)).collect()
    }

    pub fn get_objects_in_layer(&self, layer_index: usize) -> Vec<&WorldObject> {
        self.objects.iter().filter(|o| o.layer_index == layer_index).collect()
    }
}
