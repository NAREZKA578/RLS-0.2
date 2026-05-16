use crate::world::types::{City, Layer, World, WorldObject};
use std::path::Path;
use anyhow::{Context, Result};

pub struct WorldLoader;

impl WorldLoader {
    pub fn load(path: &Path) -> Result<World> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;
        
        let world: World = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("TOML parse error: {}", e))?;
        
        tracing::info!(
            "Loaded world '{}' with {} layers, {} cities, {} objects",
            world.name,
            world.layers.len(),
            world.cities.len(),
            world.objects.len()
        );
        
        Ok(world)
    }

    pub fn save(world: &World, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(world)
            .context("Failed to serialize world")?;
        std::fs::write(path, content)
            .with_context(|| format!("Failed to write world file: {}", path.display()))?;
        tracing::info!("World saved to: {}", path.display());
        Ok(())
    }

    pub fn load_or_default(path: &Path) -> World {
        if path.exists() {
            match Self::load(path) {
                Ok(w) => w,
                Err(e) => {
                    tracing::warn!("Failed to load world {:?}: {}, using default", path, e);
                    Self::create_default()
                }
            }
        } else {
            tracing::warn!("World file not found at {:?}, using default", path);
            Self::create_default()
        }
    }

    pub fn create_default() -> World {
        World {
            name: "Новосибирск".to_string(),
            seed: 42,
            layers: vec![
                Layer {
                    name: "Ground".to_string(),
                    height: 0.0,
                    tile_size: 32,
                    chunks_x: 8,
                    chunks_z: 8,
                    visible: true,
                    locked: false,
                },
                Layer {
                    name: "City".to_string(),
                    height: 0.1,
                    tile_size: 32,
                    chunks_x: 4,
                    chunks_z: 4,
                    visible: true,
                    locked: true,
                },
            ],
            cities: vec![
                City {
                    id: "novosibirsk_center".to_string(),
                    name: "Новосибирск".to_string(),
                    position: [0.0, 0.0, 0.0],
                    size: 500.0,
                    layer_index: 1,
                },
            ],
            objects: vec![
                WorldObject {
                    id: "uaz_vehicle".to_string(),
                    name: "UAZ".to_string(),
                    object_type: "vehicle".to_string(),
                    position: [0.0, 0.0, 0.0],
                    rotation: [0.0, 0.0, 0.0],
                    scale: [1.0, 1.0, 1.0],
                    layer_index: 0,
                    city_id: None,
                    static_mesh: Some("uaz_model.obj".to_string()),
                },
            ],
        }
    }
}