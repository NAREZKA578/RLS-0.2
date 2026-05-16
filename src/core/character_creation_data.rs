use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct University {
    pub name: String,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Universities {
    #[serde(rename = "ngtu")]
    pub ngtu: University,
    #[serde(rename = "ngu")]
    pub ngu: University,
    #[serde(rename = "ngmu")]
    pub ngmu: University,
    #[serde(rename = "ngasu")]
    pub ngasu: University,
    #[serde(rename = "ngpu")]
    pub ngpu: University,
    #[serde(rename = "sibguti")]
    pub sibguti: University,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Specialties {
    pub engineering: String,
    pub mechanics: String,
    pub economics: String,
    pub law: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpecialtyRow {
    pub engineering: f64,
    pub mechanics: f64,
    pub economics: f64,
    pub law: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CapitalTable {
    pub ngtu: SpecialtyRow,
    pub ngu: SpecialtyRow,
    pub ngmu: SpecialtyRow,
    pub ngasu: SpecialtyRow,
    pub ngpu: SpecialtyRow,
    pub sibguti: SpecialtyRow,
}

impl CapitalTable {
    pub fn get(&self, university_key: &str, specialty_key: &str) -> f64 {
        let row = match university_key {
            "НГТУ" => &self.ngtu,
            "НГУ" => &self.ngu,
            "НГМУ" => &self.ngmu,
            "НГАСУ" => &self.ngasu,
            "НГПУ" => &self.ngpu,
            "СибГУТИ" => &self.sibguti,
            _ => return 50000.0,
        };
        match specialty_key {
            "Инженерия" => row.engineering,
            "Механика" => row.mechanics,
            "Экономика" => row.economics,
            "Право" => row.law,
            _ => 50000.0,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Region {
    pub name: String,
    pub position: [f64; 3],
    pub color: [f32; 3],
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColorEntry {
    pub name: String,
    pub rgb: [f32; 3],
}

#[derive(Debug, Clone, Deserialize)]
pub struct ColorList {
    pub colors: Vec<ColorEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CharacterCreationData {
    pub universities: Universities,
    pub specialties: Specialties,
    pub capital_table: CapitalTable,
    pub regions: Vec<Region>,
    pub skin_colors: ColorList,
    pub hair_colors: ColorList,
}

impl CharacterCreationData {
    pub fn load(path: &Path) -> Option<Self> {
        let content = std::fs::read_to_string(path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn get_universities_list(&self) -> Vec<&str> {
        vec![
            &self.universities.ngtu.code,
            &self.universities.ngu.code,
            &self.universities.ngmu.code,
            &self.universities.ngasu.code,
            &self.universities.ngpu.code,
            &self.universities.sibguti.code,
        ]
    }

    pub fn get_specialties_list(&self) -> Vec<&str> {
        vec![
            &self.specialties.engineering,
            &self.specialties.mechanics,
            &self.specialties.economics,
            &self.specialties.law,
        ]
    }

    pub fn get_capital(&self, university: &str, specialty: &str) -> f64 {
        self.capital_table.get(university, specialty)
    }

    pub fn get_regions(&self) -> &[Region] {
        &self.regions
    }

    pub fn get_skin_colors(&self) -> Vec<[f32; 3]> {
        self.skin_colors.colors.iter().map(|c| c.rgb).collect()
    }

    pub fn get_hair_colors(&self) -> Vec<[f32; 3]> {
        self.hair_colors.colors.iter().map(|c| c.rgb).collect()
    }
}