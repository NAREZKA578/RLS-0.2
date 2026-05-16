use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct AppPaths {
    pub config_dir: PathBuf,
    pub save_dir: PathBuf,
    pub logs_dir: PathBuf,
    pub assets_dir: PathBuf,
    pub fonts_dir: PathBuf,
    pub textures_dir: PathBuf,
    pub audio_dir: PathBuf,
    pub shaders_dir: PathBuf,
    pub data_dir: PathBuf,
    pub character_creation_data_path: PathBuf,
}

impl AppPaths {
    pub fn resolve() -> Result<Self> {
        let cwd = std::env::current_dir().context("Failed to get current directory")?;
        let exe_dir = std::env::current_exe()
            .context("Failed to get executable path")?
            .parent()
            .context("Failed to get parent directory")?
            .to_path_buf();

        let mut assets_dir = cwd.join("assets");
        if !assets_dir.is_dir() {
            assets_dir = exe_dir.join("assets");
        }
        if !assets_dir.is_dir() {
            if let Some(parent) = exe_dir.parent() {
                let candidate = parent.join("assets");
                if candidate.is_dir() {
                    assets_dir = candidate;
                }
            }
        }
        if !assets_dir.is_dir() {
            if let Some(grandparent) = exe_dir.parent().and_then(|p| p.parent()) {
                let candidate = grandparent.join("assets");
                if candidate.is_dir() {
                    assets_dir = candidate;
                }
            }
        }

        let config_dir = if let Some(data_dir) = dirs::config_dir() {
            data_dir.join("RTGC")
        } else {
            cwd.join("config")
        };

        Ok(Self {
            config_dir: config_dir.clone(),
            save_dir: config_dir.join("saves"),
            logs_dir: config_dir.join("logs"),
            assets_dir: assets_dir.clone(),
            fonts_dir: assets_dir.join("fonts"),
            textures_dir: assets_dir.join("textures"),
            audio_dir: assets_dir.join("audio"),
            shaders_dir: assets_dir.join("shaders"),
            data_dir: assets_dir.join("data"),
            character_creation_data_path: config_dir.join("character_creation_data.toml"),
        })
    }

    pub fn ensure_directories(&self) -> Result<()> {
        std::fs::create_dir_all(&self.config_dir)?;
        std::fs::create_dir_all(&self.save_dir)?;
        std::fs::create_dir_all(&self.logs_dir)?;
        Ok(())
    }
}
