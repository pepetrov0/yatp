use std::path::PathBuf;

use crate::{dictionary::Dictionary, format, packer::Packer, Rect};

#[derive(Debug)]
pub struct Context {
    packer: Packer,
    dictionary: Dictionary,
}

impl Context {
    pub fn new() -> Self {
        Self {
            packer: Packer::new(),
            dictionary: Dictionary::new(),
        }
    }

    pub fn pack(&mut self, path: &PathBuf, gap: u32) -> Option<Rect> {
        let image = image::open(path).ok()?;
        loop {
            if let Some(rect) = self.packer.pack(&image, gap) {
                self.dictionary.record(path, &rect);
                return Some(rect);
            } else if !self.packer.enlarge() {
                break;
            }
        }

        None
    }

    pub fn save_to_file(
        &self,
        name: &str,
        image: format::ImageFormat,
        dict: Option<format::DictionaryFormat>,
    ) -> anyhow::Result<()> {
        self.packer.save(name, image)?;

        if let Some(dict) = dict {
            self.dictionary.save(name, dict)?;
        }
        Ok(())
    }
}
