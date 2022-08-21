use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use context::Context;
use itertools::Itertools;

mod cli;
mod context;
mod dictionary;
mod format;
mod packer;

pub type Size = euclid::Size2D<u32, u32>;
pub type Point = euclid::Point2D<u32, u32>;
pub type Rect = euclid::Rect<u32, u32>;
pub type Color = image::Rgba<u8>;

fn main() {
    let cli = Cli::parse();
    let mut context = Context::new(Size::new(cli.width, cli.height));

    let input = cli.inputs;
    let gap = cli.gap;
    let name = cli.output;
    let image = cli.image;
    let dict = cli.dict;

    let images = input
        .into_iter()
        .flat_map(expand_path)
        .filter(|v| v.is_file())
        .filter_map(|v| {
            let image = image::open(v.clone()).ok()?;
            Some((v, image.height()))
        })
        .sorted_by(|a, b| (b.1).cmp(&a.1))
        .map(|v| v.0);

    for file in images {
        match context.pack(&file, gap).is_some() {
            true => println!("packed: {}", file.to_string_lossy()),
            false => println!("could not pack: {}", file.to_string_lossy()),
        }
    }

    context.save_to_file(&name, image, dict).unwrap();
}

fn expand_path(path: PathBuf) -> Vec<PathBuf> {
    if !path.is_dir() {
        return vec![path];
    }

    path.read_dir()
        .map(|v| v.collect_vec())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|v| v.ok())
        .flat_map(|v| expand_path(v.path()))
        .collect_vec()
}
