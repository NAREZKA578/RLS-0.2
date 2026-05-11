use bytemuck::{Pod, Zeroable};
use glam::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SceneVertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: [f32; 3],
}

unsafe impl Zeroable for SceneVertex {}
unsafe impl Pod for SceneVertex {}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<SceneVertex>,
    pub indices: Vec<u32>,
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn with_capacity(vertices: usize, indices: usize) -> Self {
        Self {
            vertices: Vec::with_capacity(vertices),
            indices: Vec::with_capacity(indices),
        }
    }
}

impl Default for SceneVertex {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            normal: Vec3::Y,
            color: [0.8, 0.8, 0.8],
        }
    }
}

impl SceneVertex {
    pub fn new(position: Vec3, normal: Vec3, color: [f32; 3]) -> Self {
        Self {
            position,
            normal,
            color,
        }
    }

    pub fn with_color(position: Vec3, color: [f32; 3]) -> Self {
        Self {
            position,
            normal: Vec3::Y,
            color,
        }
    }
}

pub fn create_box(width: f32, height: f32, depth: f32, color: [f32; 3]) -> Mesh {
    let hw = width / 2.0;
    let hh = height / 2.0;
    let hd = depth / 2.0;

    let mut mesh = Mesh::with_capacity(24, 36);

    let vertices = [
        (Vec3::new(-hw, -hh, -hd), Vec3::new(0.0, -1.0, 0.0)),
        (Vec3::new(hw, -hh, -hd), Vec3::new(0.0, -1.0, 0.0)),
        (Vec3::new(hw, -hh, hd), Vec3::new(0.0, -1.0, 0.0)),
        (Vec3::new(-hw, -hh, hd), Vec3::new(0.0, -1.0, 0.0)),
        (Vec3::new(-hw, hh, -hd), Vec3::new(0.0, 1.0, 0.0)),
        (Vec3::new(hw, hh, -hd), Vec3::new(0.0, 1.0, 0.0)),
        (Vec3::new(hw, hh, hd), Vec3::new(0.0, 1.0, 0.0)),
        (Vec3::new(-hw, hh, hd), Vec3::new(0.0, 1.0, 0.0)),
        (Vec3::new(-hw, -hh, hd), Vec3::new(0.0, 0.0, 1.0)),
        (Vec3::new(hw, -hh, hd), Vec3::new(0.0, 0.0, 1.0)),
        (Vec3::new(hw, hh, hd), Vec3::new(0.0, 0.0, 1.0)),
        (Vec3::new(-hw, hh, hd), Vec3::new(0.0, 0.0, 1.0)),
        (Vec3::new(-hw, -hh, -hd), Vec3::new(0.0, 0.0, -1.0)),
        (Vec3::new(hw, -hh, -hd), Vec3::new(0.0, 0.0, -1.0)),
        (Vec3::new(hw, hh, -hd), Vec3::new(0.0, 0.0, -1.0)),
        (Vec3::new(-hw, hh, -hd), Vec3::new(0.0, 0.0, -1.0)),
        (Vec3::new(hw, -hh, -hd), Vec3::new(1.0, 0.0, 0.0)),
        (Vec3::new(hw, -hh, hd), Vec3::new(1.0, 0.0, 0.0)),
        (Vec3::new(hw, hh, hd), Vec3::new(1.0, 0.0, 0.0)),
        (Vec3::new(hw, hh, -hd), Vec3::new(1.0, 0.0, 0.0)),
        (Vec3::new(-hw, -hh, -hd), Vec3::new(-1.0, 0.0, 0.0)),
        (Vec3::new(-hw, -hh, hd), Vec3::new(-1.0, 0.0, 0.0)),
        (Vec3::new(-hw, hh, hd), Vec3::new(-1.0, 0.0, 0.0)),
        (Vec3::new(-hw, hh, -hd), Vec3::new(-1.0, 0.0, 0.0)),
    ];

    for (pos, norm) in vertices.iter() {
        mesh.vertices.push(SceneVertex::new(*pos, *norm, color));
    }

    let indices: [u32; 36] = [
        0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 8, 10, 9, 8, 11, 10, 12, 14, 13, 12, 15, 14, 16, 18,
        17, 16, 19, 18, 20, 22, 21, 20, 23, 22,
    ];
    mesh.indices.extend_from_slice(&indices);

    mesh
}

pub fn create_ground_plane(size: f32, color: [f32; 3]) -> Mesh {
    let hs = size / 2.0;
    let mut mesh = Mesh::with_capacity(4, 6);

    mesh.vertices
        .push(SceneVertex::new(Vec3::new(-hs, 0.0, -hs), Vec3::Y, color));
    mesh.vertices
        .push(SceneVertex::new(Vec3::new(hs, 0.0, -hs), Vec3::Y, color));
    mesh.vertices
        .push(SceneVertex::new(Vec3::new(hs, 0.0, hs), Vec3::Y, color));
    mesh.vertices
        .push(SceneVertex::new(Vec3::new(-hs, 0.0, hs), Vec3::Y, color));

    mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);

    mesh
}

pub fn create_uaz_body_mesh() -> Mesh {
    let body_color: [f32; 3] = [0.75, 0.72, 0.68];
    let mut mesh = Mesh::with_capacity(4, 6);

    mesh.vertices.push(SceneVertex::new(
        Vec3::new(-2.0, 0.5, -4.0),
        Vec3::new(0.0, 1.0, 0.0),
        body_color,
    ));
    mesh.vertices.push(SceneVertex::new(
        Vec3::new(2.0, 0.5, -4.0),
        Vec3::new(0.0, 1.0, 0.0),
        body_color,
    ));
    mesh.vertices.push(SceneVertex::new(
        Vec3::new(2.0, 0.5, 4.0),
        Vec3::new(0.0, 1.0, 0.0),
        body_color,
    ));
    mesh.vertices.push(SceneVertex::new(
        Vec3::new(-2.0, 0.5, 4.0),
        Vec3::new(0.0, 1.0, 0.0),
        body_color,
    ));

    mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);

    mesh
}
