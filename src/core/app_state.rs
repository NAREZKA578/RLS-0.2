use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Splash,
    MainMenu,
    Settings { return_to: Box<AppState> },
    CharacterCreation,
    SaveSelect,
    Loading { character_data: Box<CharacterData> },
    Playing,
    PauseMenu,
}

impl AppState {
    pub fn state_name(&self) -> &'static str {
        match self {
            AppState::Splash => "Splash",
            AppState::MainMenu => "MainMenu",
            AppState::Settings { .. } => "Settings",
            AppState::CharacterCreation => "CharacterCreation",
            AppState::SaveSelect => "SaveSelect",
            AppState::Loading { .. } => "Loading",
            AppState::Playing => "Playing",
            AppState::PauseMenu => "PauseMenu",
        }
    }

    pub fn valid_transitions(&self) -> &[&'static str] {
        match self {
            AppState::Splash => &["MainMenu"],
            AppState::MainMenu => &["CharacterCreation", "SaveSelect", "Settings"],
            AppState::CharacterCreation => &["Loading", "MainMenu"],
            AppState::SaveSelect => &["Loading", "MainMenu"],
            AppState::Loading { .. } => &["Playing"],
            AppState::Playing => &["PauseMenu", "MainMenu"],
            AppState::PauseMenu => &["Playing", "Settings", "MainMenu"],
            AppState::Settings { .. } => &["MainMenu", "PauseMenu", "Playing"],
        }
    }

    pub fn can_transition_to(&self, target: &AppState) -> bool {
        self.valid_transitions().contains(&target.state_name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharacterData {
    pub id: String,
    pub gender: Gender,
    pub height_m: f32,
    pub skin_color: u8,
    pub face_index: u8,
    pub hair_index: u8,
    pub hair_color: [f32; 3],
    pub university_id: String,
    pub specialty: String,
    pub start_capital: f64,
    pub uaz_color: [f32; 3],
    pub start_region: String,
    pub start_pos: [f64; 3],
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

impl Default for CharacterData {
    fn default() -> Self {
        Self {
            id: String::new(),
            gender: Gender::Male,
            height_m: 1.80,
            skin_color: 0,
            face_index: 0,
            hair_index: 0,
            hair_color: [0.2, 0.15, 0.1],
            university_id: String::new(),
            specialty: String::new(),
            start_capital: 50000.0,
            uaz_color: [0.9, 0.9, 0.9],
            start_region: String::from("center"),
            start_pos: [0.0, 0.0, 0.0],
        }
    }
}
