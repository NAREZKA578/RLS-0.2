use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectType {
    pub name: String,
}

impl ObjectType {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectTypeEnum {
    StaticMesh,
    VehicleSpawn,
    ContractPoint,
    Npc,
    Trigger,
}

impl std::fmt::Display for ObjectTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectTypeEnum::StaticMesh => write!(f, "static_mesh"),
            ObjectTypeEnum::VehicleSpawn => write!(f, "vehicle_spawn"),
            ObjectTypeEnum::ContractPoint => write!(f, "contract_point"),
            ObjectTypeEnum::Npc => write!(f, "npc"),
            ObjectTypeEnum::Trigger => write!(f, "trigger"),
        }
    }
}

impl From<&str> for ObjectTypeEnum {
    fn from(s: &str) -> Self {
        match s {
            "static_mesh" => ObjectTypeEnum::StaticMesh,
            "vehicle_spawn" => ObjectTypeEnum::VehicleSpawn,
            "contract_point" => ObjectTypeEnum::ContractPoint,
            "npc" => ObjectTypeEnum::Npc,
            "trigger" => ObjectTypeEnum::Trigger,
            _ => ObjectTypeEnum::StaticMesh,
        }
    }
}

pub struct Heightmap {
    pub width: i32,
    pub depth: i32,
    pub world_width: f32,
    pub world_depth: f32,
    heights: Vec<f32>,
}

impl Heightmap {
    pub fn new(cells_x: i32, cells_z: i32, world_w: f32, world_d: f32) -> Self {
        let heights = vec![0.0f32; ((cells_x + 1) * (cells_z + 1)) as usize];
        Self {
            width: cells_x,
            depth: cells_z,
            world_width: world_w,
            world_depth: world_d,
            heights,
        }
    }

    pub fn set_height(&mut self, x: i32, z: i32, height: f32) {
        if x >= 0 && x <= self.width && z >= 0 && z <= self.depth {
            let idx = (z * (self.width + 1) + x) as usize;
            if idx < self.heights.len() {
                self.heights[idx] = height;
            }
        }
    }

    pub fn sample(&self, world_x: f32, world_z: f32) -> f32 {
        let half_w = self.world_width / 2.0;
        let half_d = self.world_depth / 2.0;
        let step_x = self.world_width / self.width as f32;
        let step_z = self.world_depth / self.depth as f32;

        let nx = ((world_x + half_w) / step_x).clamp(0.0, self.width as f32 - 0.001);
        let nz = ((world_z + half_d) / step_z).clamp(0.0, self.depth as f32 - 0.001);

        let ix = nx as i32;
        let iz = nz as i32;
        let fx = nx - ix as f32;
        let fz = nz - iz as f32;

        let h00 = self.get(ix, iz);
        let h10 = self.get(ix + 1, iz);
        let h01 = self.get(ix, iz + 1);
        let h11 = self.get(ix + 1, iz + 1);

        h00 * (1.0 - fx) * (1.0 - fz)
            + h10 * fx * (1.0 - fz)
            + h01 * (1.0 - fx) * fz
            + h11 * fx * fz
    }

    fn get(&self, x: i32, z: i32) -> f32 {
        let x = x.clamp(0, self.width);
        let z = z.clamp(0, self.depth);
        let idx = (z * (self.width + 1) + x) as usize;
        self.heights.get(idx).copied().unwrap_or(0.0)
    }
}