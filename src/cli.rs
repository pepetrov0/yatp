use clap::Parser;
use std::path::PathBuf;

use crate::format;

/// A `clap` parser structure
#[derive(Debug, Parser)]
#[clap(
    about = "Yet Another Texture Packer - a small and simple CLI application to pack \nmultiple textures/sprites into a texture atlas/sprite sheet."
)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    /// Input images and folders
    #[clap(value_parser, help = "Files and folders to pack")]
    pub inputs: Vec<PathBuf>,

    /// Gap between packed textures
    #[clap(
        short,
        long,
        value_parser,
        default_value_t = 0,
        help = "Gap between packed textures"
    )]
    pub gap: u32,

    /// Image format
    #[clap(short, long, arg_enum, value_parser, default_value_t = format::ImageFormat::Png, help = "Output format of atlas")]
    pub image: format::ImageFormat,

    /// Dictionary format, if any
    #[clap(
        short,
        long,
        arg_enum,
        value_parser,
        help = "Output format of dictionary (optional)"
    )]
    pub dict: Option<format::DictionaryFormat>,

    /// Name of output image and dictionary
    #[clap(short, long, value_parser, default_value_t = String::from("atlas"), help = "Name of output file(s)")]
    pub output: String,

    /// Width of the output atlas
    #[clap(
        short,
        long,
        value_parser,
        default_value_t = 1024,
        help = "Width of output atlas"
    )]
    pub width: u32,

    /// Height of the output atlas
    #[clap(
        short,
        long,
        value_parser,
        default_value_t = 1024,
        help = "Height of output atlas"
    )]
    pub height: u32,
}
