use crate::graphics::game_renderer::camera::{SceneCamera, Transform};
use crate::graphics::game_renderer::mesh::Mesh;
use crate::graphics::game_renderer::obj_loader::ObjLoader;
use crate::graphics::game_renderer::shaders::{DebugShader, SceneShader, TerrainShader};
use crate::graphics::game_renderer::terrain::TerrainMesh;
use crate::world::{World, WorldLoader};
use glam::{Mat4, Vec3};
use glow::HasContext;
use std::path::{Path, PathBuf};

struct WorldObjectRenderData {
    vao: glow::NativeVertexArray,
    index_count: u32,
    transform: Transform,
    visible: bool,
    layer_index: usize,
}

pub struct GameSceneRenderer {
    assets_dir: PathBuf,
    camera: SceneCamera,
    scene_program: glow::NativeProgram,
    #[expect(dead_code)]
    terrain_program: glow::NativeProgram,
    #[expect(dead_code)]
    debug_program: glow::NativeProgram,
    terrain_vao: glow::NativeVertexArray,
    #[expect(dead_code)]
    terrain_vbo: glow::NativeBuffer,
    #[expect(dead_code)]
    terrain_ibo: glow::NativeBuffer,
    terrain_index_count: u32,
    vehicle_mesh: Mesh,
    #[expect(dead_code)]
    vehicle_vao: glow::NativeVertexArray,
    #[expect(dead_code)]
    vehicle_vbo: glow::NativeBuffer,
    #[expect(dead_code)]
    vehicle_ibo: glow::NativeBuffer,
    #[expect(dead_code)]
    vehicle_index_count: u32,
    light_dir: Vec3,
    world: Option<World>,
    world_objects: Vec<WorldObjectRenderData>,
    #[expect(dead_code)]
    debug_vao: glow::NativeVertexArray,
    #[expect(dead_code)]
    debug_vbo: glow::NativeBuffer,
    #[expect(dead_code)]
    debug_ibo: glow::NativeBuffer,
    #[expect(dead_code)]
    debug_index_count: u32,
    #[expect(dead_code)]
    test_vao: glow::NativeVertexArray,
    #[expect(dead_code)]
    test_vbo: glow::NativeBuffer,
}

impl Default for GameSceneRenderer {
    fn default() -> Self {
        panic!("GameSceneRenderer::default() should not be called directly, use GameSceneRenderer::new()")
    }
}

impl GameSceneRenderer {
    pub fn new(gl: &glow::Context, assets_dir: PathBuf) -> Result<Self, String> {
        let camera = SceneCamera::new();

        let scene_shader = SceneShader::new();
        let scene_program =
            Self::compile_shader_program(gl, &scene_shader.vs_source, &scene_shader.fs_source)?;

        let terrain_shader = TerrainShader::new();
        let terrain_program =
            Self::compile_shader_program(gl, &terrain_shader.vs_source, &terrain_shader.fs_source)?;

        let debug_shader = DebugShader::new();
        let debug_program =
            Self::compile_shader_program(gl, &debug_shader.vs_source, &debug_shader.fs_source)?;

        let terrain = TerrainMesh::default();
        let (terrain_vao, terrain_vbo, terrain_ibo) =
            Self::create_terrain_buffers(gl, &terrain.mesh)?;
        let terrain_index_count = terrain.mesh.indices.len() as u32;

        let vehicle_path = assets_dir.join("models/uaz_model.obj");
        let vehicle_mesh = ObjLoader::load(&vehicle_path)
            .unwrap_or_else(|e| {
                tracing::warn!("Failed to load vehicle mesh: {}, using default", e);
                Mesh::new()
            });
        let (vehicle_vao, vehicle_vbo, vehicle_ibo) =
            Self::create_mesh_buffers(gl, &vehicle_mesh, false)?;
        let vehicle_index_count = vehicle_mesh.indices.len() as u32;

        let light_dir = Vec3::new(0.5, 1.0, 0.3).normalize();

        let world_path = assets_dir.join("world/default_world.toml");
        tracing::info!("Looking for world at: {:?}", world_path);
        let world = if world_path.exists() {
            match WorldLoader::load(&world_path) {
                Ok(w) => {
                    tracing::info!("World file loaded, {} objects defined", w.objects.len());
                    Some(w)
                }
                Err(e) => {
                    tracing::warn!("Failed to load world: {}, using default", e);
                    Some(WorldLoader::create_default())
                }
            }
        } else {
            tracing::warn!("World file not found at {:?}, using default world", world_path);
            Some(WorldLoader::create_default())
        };

        let world_objects = if let Some(ref w) = world {
            let objs = Self::load_world_objects(gl, w, &assets_dir);
            tracing::info!("Loaded {} world objects for rendering", objs.len());
            objs
        } else {
            tracing::warn!("No world loaded!");
            Vec::new()
        };

        let (debug_vao, debug_vbo, debug_ibo, debug_index_count, test_vao, test_vbo) = Self::create_debug_geometry(gl)?;

        tracing::info!("GameSceneRenderer initialized");

        Ok(Self {
            assets_dir,
            camera,
            scene_program,
            terrain_program,
            debug_program,
            terrain_vao,
            terrain_vbo,
            terrain_ibo,
            terrain_index_count,
            vehicle_mesh,
            vehicle_vao,
            vehicle_vbo,
            vehicle_ibo,
            vehicle_index_count,
            light_dir,
            world,
            world_objects,
            debug_vao,
            debug_vbo,
            debug_ibo,
            debug_index_count,
            test_vao,
            test_vbo,
        })
    }

    fn load_world_objects(gl: &glow::Context, world: &World, assets_dir: &Path) -> Vec<WorldObjectRenderData> {
        let mut objects = Vec::new();
        
        for obj in &world.objects {
            if let Some(ref mesh_path) = obj.static_mesh {
                let full_path = assets_dir.join("models").join(mesh_path);
                if let Ok(mesh) = ObjLoader::load(&full_path) {
                    let index_count = mesh.indices.len() as u32;
                    match Self::create_mesh_buffers(gl, &mesh, false) {
                        Ok((vao, _, _)) => {
                            let position = Vec3::new(obj.position[0], obj.position[1], obj.position[2]);
                            let transform = Transform::new(position);
                            objects.push(WorldObjectRenderData {
                                vao,
                                index_count,
                                transform,
                                visible: true,
                                layer_index: obj.layer_index,
                            });
                            tracing::info!("Loaded world object: {} at {:?}", obj.name, obj.position);
                        }
                        Err(e) => {
                            tracing::warn!("Failed to create buffers for {}: {}", obj.name, e);
                        }
                    }
                } else {
                    tracing::warn!("Failed to load mesh for {}: {:?}", obj.name, mesh_path);
                }
            }
        }
        
        tracing::info!("Loaded {} world objects", objects.len());
        objects
    }

    fn compile_shader_program(
        gl: &glow::Context,
        vs_source: &str,
        fs_source: &str,
    ) -> Result<glow::NativeProgram, String> {
        unsafe {
            let vs = gl
                .create_shader(glow::VERTEX_SHADER)
                .map_err(|e| e.to_string())?;
            gl.shader_source(vs, vs_source);
            gl.compile_shader(vs);
            if !gl.get_shader_compile_status(vs) {
                let log = gl.get_shader_info_log(vs);
                gl.delete_shader(vs);
                return Err(format!("VS: {}", log));
            }

            let fs = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .map_err(|e| e.to_string())?;
            gl.shader_source(fs, fs_source);
            gl.compile_shader(fs);
            if !gl.get_shader_compile_status(fs) {
                let log = gl.get_shader_info_log(fs);
                gl.delete_shader(fs);
                return Err(format!("FS: {}", log));
            }

            let program = gl.create_program().map_err(|e| e.to_string())?;
            gl.attach_shader(program, vs);
            gl.attach_shader(program, fs);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                let log = gl.get_program_info_log(program);
                gl.delete_program(program);
                return Err(format!("Link: {}", log));
            }

            gl.delete_shader(vs);
            gl.delete_shader(fs);

            Ok(program)
        }
    }

    fn create_mesh_buffers(
        gl: &glow::Context,
        mesh: &Mesh,
        _interleaved: bool,
    ) -> Result<
        (
            glow::NativeVertexArray,
            glow::NativeBuffer,
            glow::NativeBuffer,
        ),
        String,
    > {
        unsafe {
            let vao = gl.create_vertex_array().map_err(|e| e.to_string())?;
            let vbo = gl.create_buffer().map_err(|e| e.to_string())?;
            let ibo = gl.create_buffer().map_err(|e| e.to_string())?;

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let vertex_bytes: &[u8] = bytemuck::cast_slice(&mesh.vertices);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertex_bytes, glow::STATIC_DRAW);

            let stride =
                std::mem::size_of::<crate::graphics::game_renderer::mesh::SceneVertex>() as i32;
            let offset_pos = 0_i32;
            let offset_norm = 12_i32;
            let offset_color = 24_i32;

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, offset_pos);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, stride, offset_norm);
            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 3, glow::FLOAT, false, stride, offset_color);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
            let index_bytes: &[u8] = bytemuck::cast_slice(&mesh.indices);
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, index_bytes, glow::STATIC_DRAW);

            gl.bind_vertex_array(None);

            tracing::debug!("Created mesh buffers: vao={:?}, vbo={:?}, ibo={:?}, vertices={}, indices={}", 
                vao, vbo, ibo, mesh.vertices.len(), mesh.indices.len());

            Ok((vao, vbo, ibo))
        }
    }

    fn create_terrain_buffers(
        gl: &glow::Context,
        mesh: &Mesh,
    ) -> Result<
        (
            glow::NativeVertexArray,
            glow::NativeBuffer,
            glow::NativeBuffer,
        ),
        String,
    > {
        unsafe {
            let vao = gl.create_vertex_array().map_err(|e| e.to_string())?;
            let vbo = gl.create_buffer().map_err(|e| e.to_string())?;
            let ibo = gl.create_buffer().map_err(|e| e.to_string())?;

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let vertex_bytes: &[u8] = bytemuck::cast_slice(&mesh.vertices);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertex_bytes, glow::STATIC_DRAW);

            let stride =
                std::mem::size_of::<crate::graphics::game_renderer::mesh::SceneVertex>() as i32;

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, stride, 24);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
            let index_bytes: &[u8] = bytemuck::cast_slice(&mesh.indices);
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, index_bytes, glow::STATIC_DRAW);

            gl.bind_vertex_array(None);

            tracing::debug!("Created terrain buffers: vao={:?}, indices={}", vao, mesh.indices.len());

            Ok((vao, vbo, ibo))
        }
    }

    fn create_debug_geometry(gl: &glow::Context) -> Result<(glow::NativeVertexArray, glow::NativeBuffer, glow::NativeBuffer, u32, glow::NativeVertexArray, glow::NativeBuffer), String> {
        unsafe {
            let vao = gl.create_vertex_array().map_err(|e| e.to_string())?;
            let vbo = gl.create_buffer().map_err(|e| e.to_string())?;
            let ibo = gl.create_buffer().map_err(|e| e.to_string())?;

            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            let positions: [f32; 18] = [
                0.0, 0.0, 0.0,
                100.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                0.0, 100.0, 0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 100.0,
            ];
            let bytes: &[u8] = bytemuck::cast_slice(&positions);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytes, glow::STATIC_DRAW);

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
            let indices: [u32; 6] = [0, 1, 2, 3, 4, 5];
            let index_bytes: &[u8] = bytemuck::cast_slice(&indices);
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, index_bytes, glow::STATIC_DRAW);

            gl.bind_vertex_array(None);

            let terrain_vao = gl.create_vertex_array().map_err(|e| e.to_string())?;
            let terrain_vbo = gl.create_buffer().map_err(|e| e.to_string())?;

            gl.bind_vertex_array(Some(terrain_vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(terrain_vbo));

            let terrain_verts: [f32; 36] = [
                -250.0, 0.0, -250.0,  0.0, 1.0, 0.0,  0.25, 0.55, 0.15,  // трава - зелёная как в жизни
                250.0, 0.0, -250.0,   0.0, 1.0, 0.0,  0.22, 0.5, 0.12,
                250.0, 0.0, 250.0,    0.0, 1.0, 0.0,  0.28, 0.58, 0.18,
                -250.0, 0.0, 250.0,   0.0, 1.0, 0.0,  0.25, 0.55, 0.15,
            ];
            let terrain_bytes: &[u8] = bytemuck::cast_slice(&terrain_verts);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, terrain_bytes, glow::STATIC_DRAW);

            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 36, 0);
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 36, 12);
            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 3, glow::FLOAT, false, 36, 24);

            gl.bind_vertex_array(None);

            tracing::info!("Created debug axes and terrain: vao={:?}", vao);
            Ok((vao, vbo, ibo, 6, terrain_vao, terrain_vbo))
        }
    }

    pub fn set_vehicle_mesh(&mut self, mesh: Mesh) {
        self.vehicle_mesh = mesh;
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.camera.set_aspect(width, height);
    }

    pub fn update(&mut self, _dt: f32) {}

    pub fn set_character_height(&mut self, height_m: f32) {
        self.camera.position.y = height_m - 0.08;
    }

    pub fn render(&mut self, gl: &glow::Context) {
        let view_matrix = self.camera.view_matrix();
        let proj_matrix = self.camera.projection_matrix();

        unsafe {
            gl.clear_color(0.53, 0.81, 0.92, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            gl.enable(glow::DEPTH_TEST);
            gl.enable(glow::CULL_FACE);

            gl.use_program(Some(self.scene_program));
            if let Some(loc) = gl.get_uniform_location(self.scene_program, "u_view") {
                gl.uniform_matrix_4_f32_slice(Some(&loc), false, &view_matrix.to_cols_array());
            }
            if let Some(loc) = gl.get_uniform_location(self.scene_program, "u_projection") {
                gl.uniform_matrix_4_f32_slice(Some(&loc), false, &proj_matrix.to_cols_array());
            }
            let identity_matrix = Mat4::IDENTITY;
            if let Some(loc) = gl.get_uniform_location(self.scene_program, "u_model") {
                gl.uniform_matrix_4_f32_slice(Some(&loc), false, &identity_matrix.to_cols_array());
            }
            let light_arr = [self.light_dir.x, self.light_dir.y, self.light_dir.z];
            if let Some(loc) = gl.get_uniform_location(self.scene_program, "u_light_dir") {
                gl.uniform_3_f32_slice(Some(&loc), &light_arr);
            }
            let view_pos = [self.camera.position.x, self.camera.position.y, self.camera.position.z];
            if let Some(loc) = gl.get_uniform_location(self.scene_program, "u_view_pos") {
                gl.uniform_3_f32_slice(Some(&loc), &view_pos);
            }

            gl.bind_vertex_array(Some(self.terrain_vao));
            gl.draw_elements(glow::TRIANGLES, self.terrain_index_count as i32, glow::UNSIGNED_INT, 0);

            for obj in &self.world_objects {
                if obj.visible {
                    let model_matrix = obj.transform.matrix();
                    if let Some(loc) = gl.get_uniform_location(self.scene_program, "u_model") {
                        gl.uniform_matrix_4_f32_slice(Some(&loc), false, &model_matrix.to_cols_array());
                    }
                    gl.bind_vertex_array(Some(obj.vao));
                    gl.draw_elements(glow::TRIANGLES, obj.index_count as i32, glow::UNSIGNED_INT, 0);
                }
            }

            gl.bind_vertex_array(None);
            gl.use_program(None);
        }
    }

    pub fn move_camera(&mut self, forward: f32, right: f32) {
        self.camera.move_forward(forward);
        self.camera.move_right(right);
    }

    pub fn rotate_camera(&mut self, d_pitch: f32, d_yaw: f32) {
        self.camera.rotate(d_pitch, d_yaw);
    }

    pub fn get_world(&self) -> Option<&World> {
        self.world.as_ref()
    }

    pub fn set_layer_visible(&mut self, layer_index: usize, visible: bool) {
        for obj in &mut self.world_objects {
            if obj.layer_index == layer_index {
                obj.visible = visible;
            }
        }
    }

    pub fn reload_world(&mut self, gl: &glow::Context) {
        let world_path = self.assets_dir.join("world/default_world.toml");
        if let Ok(new_world) = WorldLoader::load(&world_path) {
            self.world = Some(new_world);
            self.world_objects = Self::load_world_objects(gl, self.world.as_ref().unwrap(), &self.assets_dir);
        }
    }
}
