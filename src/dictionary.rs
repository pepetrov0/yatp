//!
//! Structures used to store data about where a given texture was packed in the texture atlas.
//!

use std::path::PathBuf;

use serde::Serialize;

use crate::{format, Size};

/// Rectangle
#[derive(Debug, Serialize, Clone, Copy)]
pub struct Rect {
    /// Top-left X
    pub x: u32,
    /// Top-left Y
    pub y: u32,
    /// Width
    pub width: u32,
    /// Height
    pub height: u32,
}

/// Entry into a `Dictionary`
#[derive(Debug, Serialize, Clone)]
pub struct Entry {
    /// Name of the dictionary entry, usually the file stem with a suffix
    pub name: String,
    /// Path of the dictionary entry
    pub path: String,
    /// Rectangle
    pub rect: Rect,
}

/// Dictionary containing data about the packed textures and texture atlas, serialized into a given format.
#[derive(Debug, Serialize)]
pub struct Dictionary {
    /// Width of the atlas
    width: u32,
    /// Height of the atlas
    height: u32,
    /// List of entries recorded
    items: Vec<Entry>,
}

impl Dictionary {
    /// Creates a new dictionary with given atlas `size` and no entries
    pub fn new(size: Size) -> Self {
        Self {
            width: size.width,
            height: size.height,
            items: vec![],
        }
    }

    /// Records a new entry into the dictionary with provided `path` and `rect` (rectangle).
    /// NOTE: name of the entry is derived from its path and suffixes may be appended to avoid
    /// conflicting names
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

    /// Serializes the dictionary to a file with given `name` and `dict` format
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
