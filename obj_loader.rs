use crate::graphics::game_renderer::mesh::{Mesh, SceneVertex};
use glam::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct ObjLoader;

impl ObjLoader {
    pub fn load(path: &Path) -> Result<Mesh, String> {
        let file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
        let reader = BufReader::new(file);

        let mut vertices: Vec<Vec3> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut mesh_vertices: Vec<SceneVertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut current_color: [f32; 3] = [0.75, 0.72, 0.68];

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" if parts.len() >= 4 => {
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    vertices.push(Vec3::new(x, y, z));
                }
                "vn" if parts.len() >= 4 => {
                    let x: f32 = parts[1].parse().unwrap_or(0.0);
                    let y: f32 = parts[2].parse().unwrap_or(0.0);
                    let z: f32 = parts[3].parse().unwrap_or(0.0);
                    normals.push(Vec3::new(x, y, z).normalize());
                }
                "f" if parts.len() >= 4 => {
                    let mut face_indices: Vec<u32> = Vec::new();

                    for part in &parts[1..] {
                        let indices_str: Vec<&str> = part.split('/').collect();
                        if let Some(vert_idx) = indices_str.first() {
                            let idx: i32 = vert_idx.parse().unwrap_or(1);
                            let adjusted = if idx < 0 {
                                vertices.len() as i32 + idx + 1
                            } else {
                                idx
                            } as usize;
                            if adjusted > 0 && adjusted <= vertices.len() {
                                let vert = vertices[adjusted - 1];
                                let normal_idx = indices_str.get(2).and_then(|s| s.parse::<usize>().ok());
                                let normal = normal_idx
                                    .and_then(|n| if n > 0 && n <= normals.len() { Some(normals[n - 1]) } else { None })
                                    .unwrap_or(Vec3::Y);

                                let vertex = SceneVertex::new(vert, normal, current_color);
                                let vertex_index = mesh_vertices.len() as u32;
                                mesh_vertices.push(vertex);
                                face_indices.push(vertex_index);
                            }
                        }
                    }

                    if face_indices.len() >= 3 {
                        for i in 1..(face_indices.len() - 1) {
                            indices.push(face_indices[0]);
                            indices.push(face_indices[i]);
                            indices.push(face_indices[i + 1]);
                        }
                    }
                }
                "o" => {
                    if parts.len() > 1 {
                        let name = parts[1].to_lowercase();
                        if name.contains("body") || name.contains("cabin") || name.contains("trunk") {
                            current_color = [0.72, 0.68, 0.60];
                        } else if name.contains("wheel") || name.contains("hub") {
                            current_color = [0.12, 0.12, 0.12];
                        } else if name.contains("bumper") {
                            current_color = [0.35, 0.35, 0.35];
                        } else if name.contains("glass") || name.contains("windshield") {
                            current_color = [0.3, 0.4, 0.5];
                        }
                    }
                }
                _ => {}
            }
        }

        if mesh_vertices.is_empty() {
            return Err("No vertices loaded from OBJ file".to_string());
        }

        tracing::info!("Loaded OBJ: {} vertices, {} triangles from {}", mesh_vertices.len(), indices.len() / 3, path.display());

        Ok(Mesh {
            vertices: mesh_vertices,
            indices,
        })
    }
}