use serde::Serialize;

use crate::format;

#[derive(Debug, Serialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub path: String,
    pub rect: Rect,
}

#[derive(Debug, Serialize)]
pub struct Dictionary {
    pub items: Vec<Entry>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn pick_name(&self, name: &str) -> String {
        let exists = |name: &str| self.items.iter().any(|v| v.name == name);

        if !exists(name) {
            return String::from(name);
        }

        for i in 0.. {
            let new_name = format!("{}-{}", name, i);

            if !exists(&new_name) {
                return new_name;
            }
        }

        String::from(name)
    }

    pub fn save(&self, name: &str, dict: format::DictionaryFormat) -> anyhow::Result<()> {
        let content = match dict {
            format::DictionaryFormat::Toml => toml::to_vec(self)?,
            format::DictionaryFormat::Json => serde_json::to_vec_pretty(self)?,
            format::DictionaryFormat::Yaml => serde_yaml::to_string(self)?.into_bytes(),
            format::DictionaryFormat::Ron => ron::to_string(self)?.into_bytes(),
        };
        std::fs::write(format!("{}.{}", name, dict.ext()), content)?;
        Ok(())
    }
}
