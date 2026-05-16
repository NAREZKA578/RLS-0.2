pub struct SceneShader {
    pub vs_source: String,
    pub fs_source: String,
}

impl Default for SceneShader {
    fn default() -> Self {
        Self::new()
    }
}

impl SceneShader {
    pub fn new() -> Self {
        Self {
            vs_source: r#"
                #version 330 core
                layout (location = 0) in vec3 a_position;
                layout (location = 1) in vec3 a_normal;
                layout (location = 2) in vec3 a_color;

                uniform mat4 u_model;
                uniform mat4 u_view;
                uniform mat4 u_projection;

                out vec3 v_color;
                out vec3 v_normal;
                out vec3 v_world_pos;

                void main() {
                    vec4 world_pos = u_model * vec4(a_position, 1.0);
                    v_world_pos = world_pos.xyz;
                    v_normal = mat3(u_model) * a_normal;
                    v_color = a_color;
                    gl_Position = u_projection * u_view * world_pos;
                }
            "#
            .to_string(),
            fs_source: r#"
                #version 330 core
                in vec3 v_color;
                in vec3 v_normal;
                in vec3 v_world_pos;

                out vec4 frag_color;

                uniform vec3 u_light_dir;
                uniform vec3 u_view_pos;

                void main() {
                    vec3 norm = normalize(v_normal);
                    vec3 light_dir = normalize(u_light_dir);

                    float ambient = 0.25;
                    float diff = max(dot(norm, light_dir), 0.0);

                    vec3 view_dir = normalize(u_view_pos - v_world_pos);
                    vec3 reflect_dir = reflect(-light_dir, norm);
                    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);

                    vec3 ambient_color = ambient * v_color;
                    vec3 diff_color = diff * v_color * 0.7;
                    vec3 spec_color = spec * vec3(0.5, 0.5, 0.5) * 0.3;

                    frag_color = vec4(ambient_color + diff_color + spec_color, 1.0);
                }
            "#
            .to_string(),
        }
    }
}

pub struct TerrainShader {
    pub vs_source: String,
    pub fs_source: String,
}

impl Default for TerrainShader {
    fn default() -> Self {
        Self::new()
    }
}

impl TerrainShader {
    pub fn new() -> Self {
        Self {
            vs_source: r#"
                #version 330 core
                layout (location = 0) in vec3 a_position;
                layout (location = 1) in vec3 a_color;

                uniform mat4 u_mvp;

                out vec3 v_color;

                void main() {
                    v_color = a_color;
                    gl_Position = u_mvp * vec4(a_position, 1.0);
                }
            "#
            .to_string(),
            fs_source: r#"
                #version 330 core
                in vec3 v_color;
                out vec4 frag_color;

                void main() {
                    frag_color = vec4(v_color, 1.0);
                }
            "#
            .to_string(),
        }
    }
}

pub struct DebugShader {
    pub vs_source: String,
    pub fs_source: String,
}

impl Default for DebugShader {
    fn default() -> Self {
        Self::new()
    }
}

impl DebugShader {
    pub fn new() -> Self {
        Self {
            vs_source: r#"
                #version 330 core
                layout (location = 0) in vec3 a_position;

                uniform mat4 u_mvp;

                void main() {
                    gl_Position = u_mvp * vec4(a_position, 1.0);
                }
            "#
            .to_string(),
            fs_source: r#"
                #version 330 core
                out vec4 frag_color;

                void main() {
                    frag_color = vec4(1.0, 0.0, 0.0, 1.0);
                }
            "#
            .to_string(),
        }
    }
}
