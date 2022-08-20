use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Bmp,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DictionaryFormat {
    Toml,
    Json,
    Yaml,
    Ron,
}

impl Default for ImageFormat {
    fn default() -> Self {
        Self::Png
    }
}

impl Default for DictionaryFormat {
    fn default() -> Self {
        Self::Json
    }
}

impl ImageFormat {
    pub fn ext(&self) -> &str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Bmp => "bmp",
        }
    }

    pub fn as_image_format(&self) -> image::ImageFormat {
        match self {
            ImageFormat::Png => image::ImageFormat::Png,
            ImageFormat::Jpeg => image::ImageFormat::Jpeg,
            ImageFormat::Bmp => image::ImageFormat::Bmp,
        }
    }
}

impl DictionaryFormat {
    pub fn ext(&self) -> &str {
        match self {
            DictionaryFormat::Toml => "toml",
            DictionaryFormat::Json => "json",
            DictionaryFormat::Yaml => "yaml",
            DictionaryFormat::Ron => "ron",
        }
    }
}
