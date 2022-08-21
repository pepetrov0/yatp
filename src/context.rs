use std::path::PathBuf;

use crate::{dictionary::Dictionary, format, packer::Packer, Rect, Size};

#[derive(Debug)]
pub struct Context {
    packer: Packer,
    dictionary: Dictionary,
}

impl Context {
    pub fn new(size: Size) -> Self {
        Self {
            packer: Packer::new(size),
            dictionary: Dictionary::new(size),
        }
    }

    pub fn pack(&mut self, path: &PathBuf, gap: u32) -> Option<Rect> {
        let image = image::open(path).ok()?;

        if let Some(rect) = self.packer.pack(&image, gap) {
            self.dictionary.record(path, &rect);
            return Some(rect);
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
