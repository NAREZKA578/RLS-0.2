use crossbeam_channel::{bounded, Receiver, Sender};
use std::path::PathBuf;
use std::thread;
use crate::core::app_state::CharacterData;

#[derive(Debug)]
pub enum LoadMsg {
    StageBegin { stage: u8, name: &'static str },
    Progress(f32),
    Done(Box<LoadedAssets>),
    Error(String),
}

#[derive(Debug)]
pub struct LoadedAssets {
    pub terrain_mesh: crate::graphics::game_renderer::mesh::Mesh,
    pub vehicle_mesh: crate::graphics::game_renderer::mesh::Mesh,
    pub world: crate::world::types::World,
}

pub struct BackgroundLoader {
    pub rx: Receiver<LoadMsg>,
}

impl BackgroundLoader {
    pub fn start(assets_dir: PathBuf, _character_data: CharacterData) -> Self {
        let (tx, rx) = bounded::<LoadMsg>(64);
        thread::spawn(move || {
            run_loading(tx, assets_dir);
        });
        Self { rx }
    }
}

fn run_loading(tx: Sender<LoadMsg>, assets_dir: PathBuf) {
    macro_rules! stage {
        ($n:expr, $name:expr) => {
            let _ = tx.send(LoadMsg::StageBegin { stage: $n, name: $name });
        };
    }
    macro_rules! progress {
        ($v:expr) => {
            let _ = tx.send(LoadMsg::Progress($v));
        };
    }

    stage!(1, "Генерация ландшафта");
    let terrain_mesh = crate::graphics::game_renderer::terrain::TerrainMesh::default().mesh;
    progress!(1.0);

    stage!(2, "Загрузка модели UAZ");
    let vehicle_path = assets_dir.join("models/uaz_model.obj");
    let vehicle_mesh = match crate::graphics::game_renderer::obj_loader::ObjLoader::load(&vehicle_path) {
        Ok(m) => {
            progress!(1.0);
            m
        }
        Err(e) => {
            let _ = tx.send(LoadMsg::Error(format!("UAZ: {}", e)));
            return;
        }
    };

    stage!(3, "Загрузка мирового файла");
    let world_path = assets_dir.join("world/default_world.toml");
    let world = crate::world::world_loader::WorldLoader::load_or_default(&world_path);
    progress!(1.0);

    stage!(4, "Загрузка звуков");
    std::thread::sleep(std::time::Duration::from_millis(50));
    progress!(1.0);

    let _ = tx.send(LoadMsg::Done(Box::new(LoadedAssets {
        terrain_mesh,
        vehicle_mesh,
        world,
    })));
}