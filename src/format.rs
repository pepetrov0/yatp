use clap::ValueEnum;

/// Image format
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ImageFormat {
    /// PNG
    Png,
    /// JPEG
    Jpeg,
    /// BMP
    Bmp,
}

/// Dictionary Format
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum DictionaryFormat {
    /// TOML
    Toml,
    /// JSON
    Json,
    /// YAML
    Yaml,
    /// RON
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
    /// Extension of a given image format:
    /// - png => .png
    /// - jpeg => .jpeg
    /// - bmp => .bmp
    pub fn ext(&self) -> &str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Bmp => "bmp",
        }
    }

    /// Converts an image format to `image`'s equivalent
    pub fn as_image_format(&self) -> image::ImageFormat {
        match self {
            ImageFormat::Png => image::ImageFormat::Png,
            ImageFormat::Jpeg => image::ImageFormat::Jpeg,
            ImageFormat::Bmp => image::ImageFormat::Bmp,
        }
    }
}

impl DictionaryFormat {
    /// Extension of a given dictionary format:
    /// - toml => .toml
    /// - json => .json
    /// - yaml => .yaml
    /// - ron => .ron
    pub fn ext(&self) -> &str {
        match self {
            DictionaryFormat::Toml => "toml",
            DictionaryFormat::Json => "json",
            DictionaryFormat::Yaml => "yaml",
            DictionaryFormat::Ron => "ron",
        }
    }
}
