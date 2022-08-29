use std::path::PathBuf;

use serde::Serialize;

use crate::{format, Size};

#[derive(Debug, Serialize, Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct Entry {
    pub name: String,
    pub path: String,
    pub rect: Rect,
}

#[derive(Debug, Serialize)]
pub struct Dictionary {
    width: u32,
    height: u32,
    items: Vec<Entry>,
}

impl Dictionary {
    pub fn new(size: Size) -> Self {
        Self {
            width: size.width,
            height: size.height,
            items: vec![],
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn record(&mut self, path: &PathBuf, rect: &crate::Rect) -> Entry {
        let name = path
            .file_stem()
            .or_else(|| path.file_name())
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string();

        let entry = Entry {
            name: self.pick_name(&name),
            path: path.to_string_lossy().to_string(),
            rect: self::Rect {
                x: rect.origin.x,
                y: rect.origin.y,
                width: rect.size.width,
                height: rect.size.height,
            },
        };

        self.items.push(entry.clone());
        entry
    }

    fn pick_name(&self, name: &str) -> String {
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
