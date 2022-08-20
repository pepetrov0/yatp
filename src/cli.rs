use clap::Parser;
use std::path::PathBuf;

use crate::format;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(value_parser)]
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
        help = "Output format of dictionary"
    )]
    pub dict: Option<format::DictionaryFormat>,

    #[clap(short, long, value_parser, default_value_t = String::from("atlas"), help = "Name of output file(s)")]
    pub output: String,
}
