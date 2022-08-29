use clap::Parser;
use std::path::PathBuf;

use crate::format;

#[derive(Debug, Parser)]
#[clap(
    about = "Yet Another Texture Packer - a small and simple CLI application to pack \nmultiple textures/sprites into a texture atlas/sprite sheet."
)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
pub struct Cli {
    #[clap(value_parser, help = "Files and folders to pack")]
    pub inputs: Vec<PathBuf>,

    #[clap(
        short,
        long,
        value_parser,
        default_value_t = 0,
        help = "Gap between packed textures"
    )]
    pub gap: u32,

    #[clap(short, long, arg_enum, value_parser, default_value_t = format::ImageFormat::Png, help = "Output format of atlas")]
    pub image: format::ImageFormat,

    #[clap(
        short,
        long,
        arg_enum,
        value_parser,
        help = "Output format of dictionary (optional)"
    )]
    pub dict: Option<format::DictionaryFormat>,

    #[clap(short, long, value_parser, default_value_t = String::from("atlas"), help = "Name of output file(s)")]
    pub output: String,

    #[clap(
        short,
        long,
        value_parser,
        default_value_t = 1024,
        help = "Width of output atlas"
    )]
    pub width: u32,

    #[clap(
        short,
        long,
        value_parser,
        default_value_t = 1024,
        help = "Height of output atlas"
    )]
    pub height: u32,
}
