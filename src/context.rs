use std::path::PathBuf;

use crate::{dictionary::Dictionary, format, packer::Packer, Rect, Size};

/// Bundle of a `Packer` and a `Dictionary`
#[derive(Debug)]
pub struct Context {
    packer: Packer,
    dictionary: Dictionary,
}

impl Context {
    /// Create a new context with given `size`
    pub fn new(size: Size) -> Self {
        Self {
            packer: Packer::new(size),
            dictionary: Dictionary::new(size),
        }
    }

    /// Packs image loaded from the provided `path` into the packer with the provided `gap` and
    /// records it into the dictionary. Returns the `rect` the texture was packed in if successful
    pub fn pack(&mut self, path: &PathBuf, gap: u32) -> Option<Rect> {
        let image = image::open(path).ok()?;

        if let Some(rect) = self.packer.pack(&image, gap) {
            self.dictionary.record(path, &rect);
            return Some(rect);
        }

        None
    }

    /// Saves both the packer and the dictionary to a file with given `name`, `image` format and
    /// `dictionary` format. If dictionary is `None` then the dictionary is not serialized
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
