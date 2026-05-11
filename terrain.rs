use crate::graphics::game_renderer::mesh::{Mesh, SceneVertex};
use glam::Vec3;

const CITY_WIDTH: f32 = 15000.0;
const CITY_DEPTH: f32 = 30000.0;
const _MAIN_ROAD_SPACING: f32 = 500.0;
const SIDE_STREET_SPACING: f32 = 150.0;
const ROAD_WIDTH: f32 = 15.0;

const GRASS_COLOR: [f32; 3] = [0.22, 0.52, 0.12];
const ROAD_COLOR: [f32; 3] = [0.22, 0.22, 0.22];
const SIDEWALK_COLOR: [f32; 3] = [0.52, 0.52, 0.48];
const WATER_COLOR: [f32; 3] = [0.15, 0.35, 0.55];
const PARK_COLOR: [f32; 3] = [0.18, 0.48, 0.1];
const SAND_COLOR: [f32; 3] = [0.65, 0.55, 0.45];

pub struct TerrainMesh {
    pub mesh: Mesh,
    pub width: f32,
    pub depth: f32,
    pub cells_x: i32,
    pub cells_z: i32,
}

impl Default for TerrainMesh {
    fn default() -> Self {
        Self::new(CITY_WIDTH, CITY_DEPTH, 300, 300)
    }
}

impl TerrainMesh {
    pub fn new(width: f32, depth: f32, cells_x: i32, cells_z: i32) -> Self {
        let mesh = Self::generate_novosibirsk(width, depth, cells_x, cells_z);
        Self {
            mesh,
            width,
            depth,
            cells_x,
            cells_z,
        }
    }

    fn is_water(x: f32, z: f32) -> bool {
        let ob_river_south = -11000.0;
        let ob_river_north = -13000.0;
        let ob_river_width = 800.0;

        if x > ob_river_south && x < ob_river_north + ob_river_width {
            let river_edge_z_min = -2000.0;
            let river_edge_z_max = 5000.0;

            if z > river_edge_z_min && z < river_edge_z_max {
                let river_bank_south = 50.0;
                let river_bank_north = -50.0;

                if x > river_bank_south && x < river_bank_north {
                    return true;
                }
            }
        }

        let small_river_z = 8000.0;
        let small_river_width = 100.0;
        if z > small_river_z - small_river_width && z < small_river_z + small_river_width {
            let river_bank_min = -100.0;
            let river_bank_max = 100.0;
            if x > river_bank_min && x < river_bank_max {
                return true;
            }
        }

        false
    }

    fn is_sand_beach(x: f32, _z: f32) -> bool {
        let ob_river_south = -11000.0;
        let ob_river_north = -13000.0;

        if x > ob_river_south - 100.0 && x < ob_river_north + 100.0 + 800.0 {
            let beach_zone_south = -20.0;
            let beach_zone_north = 20.0;

            if x > beach_zone_south - 80.0 && x < beach_zone_north + 80.0 {
                if x < beach_zone_south || x > beach_zone_north {
                    return true;
                }
            }
        }

        false
    }

    fn is_park(x: f32, z: f32) -> bool {
        let park1_center = (0.0, -500.0);
        let park1_radius = 400.0;
        let dx = x - park1_center.0;
        let dz = z - park1_center.1;
        if dx * dx + dz * dz < park1_radius * park1_radius {
            return true;
        }

        let park2_center = (3000.0, 2000.0);
        let park2_radius = 300.0;
        let dx2 = x - park2_center.0;
        let dz2 = z - park2_center.1;
        if dx2 * dx2 + dz2 * dz2 < park2_radius * park2_radius {
            return true;
        }

        false
    }

    fn get_road_type(x: f32, z: f32) -> i32 {
        let half_w = CITY_WIDTH / 2.0;
        let half_d = CITY_DEPTH / 2.0;
        let rel_x = x + half_w;
        let rel_z = z + half_d;

        if rel_x < 0.0 || rel_z < 0.0 || rel_x > CITY_WIDTH || rel_z > CITY_DEPTH {
            return 0;
        }

        if Self::is_water(x, z) || Self::is_sand_beach(x, z) {
            return 0;
        }

        let red_prospekt_x = half_w + 0.0;
        if (rel_x - red_prospekt_x).abs() < ROAD_WIDTH / 2.0 {
            if rel_x > 2000.0 && rel_x < 13000.0 {
                return 1;
            }
        }

        for main_x in [1000.0, 3000.0, 5000.0, 7000.0, 9000.0, 11000.0].iter() {
            let road_start = main_x - ROAD_WIDTH / 2.0;
            let road_end = main_x + ROAD_WIDTH / 2.0;
            if rel_x >= road_start && rel_x <= road_end && rel_z > 3000.0 && rel_z < 27000.0 {
                return 1;
            }
        }

        for main_z in [3000.0, 5000.0, 7000.0, 9000.0, 11000.0, 13000.0, 15000.0, 17000.0, 19000.0, 21000.0, 23000.0, 25000.0].iter() {
            let road_start = main_z - ROAD_WIDTH / 2.0;
            let road_end = main_z + ROAD_WIDTH / 2.0;
            if rel_z >= road_start && rel_z <= road_end && rel_x > 0.0 && rel_x < 15000.0 {
                if rel_z < 11000.0 || rel_z > 13000.0 || rel_x < -12000.0 || rel_x > 0.0 {
                    return 1;
                }
            }
        }

        for side_x in (500..CITY_WIDTH as usize).step_by(SIDE_STREET_SPACING as usize) {
            let road_start = side_x as f32 - ROAD_WIDTH / 4.0;
            let road_end = side_x as f32 + ROAD_WIDTH / 4.0;
            if rel_x >= road_start && rel_x <= road_end {
                return 2;
            }
        }

        for side_z in (500..CITY_DEPTH as usize).step_by(SIDE_STREET_SPACING as usize) {
            let road_start = side_z as f32 - ROAD_WIDTH / 4.0;
            let road_end = side_z as f32 + ROAD_WIDTH / 4.0;
            if rel_z >= road_start && rel_z <= road_end {
                return 2;
            }
        }

        if Self::is_park(x, z) {
            return 3;
        }

        0
    }

    fn generate_novosibirsk(width: f32, depth: f32, cells_x: i32, cells_z: i32) -> Mesh {
        let mut mesh = Mesh::with_capacity(
            ((cells_x + 1) * (cells_z + 1)) as usize,
            (cells_x * cells_z * 6) as usize,
        );

        let step_x = width / cells_x as f32;
        let step_z = depth / cells_z as f32;
        let hw = width / 2.0;
        let hd = depth / 2.0;

        for z in 0..=cells_z {
            for x in 0..=cells_x {
                let px = -hw + x as f32 * step_x;
                let pz = -hd + z as f32 * step_z;

                let road_type = Self::get_road_type(px, pz);
                let color = match road_type {
                    1 => ROAD_COLOR,
                    2 => SIDEWALK_COLOR,
                    3 => PARK_COLOR,
                    _ => {
                        if Self::is_water(px, pz) {
                            WATER_COLOR
                        } else if Self::is_sand_beach(px, pz) {
                            SAND_COLOR
                        } else {
                            GRASS_COLOR
                        }
                    }
                };

                mesh.vertices.push(SceneVertex::new(
                    Vec3::new(px, 0.0, pz),
                    Vec3::Y,
                    color,
                ));
            }
        }

        for z in 0..cells_z {
            for x in 0..cells_x {
                let i00 = (z * (cells_x + 1) + x) as u32;
                let i10 = i00 + 1;
                let i01 = i00 + (cells_x + 1) as u32;
                let i11 = i01 + 1;

                mesh.indices.extend_from_slice(&[i00, i10, i11, i00, i11, i01]);
            }
        }

        mesh
    }
}
