use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use colored::Colorize;
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

    let images = input.into_iter().flat_map(expand_path).collect_vec();

    println!("{}", "   PACKING    ".black().on_bright_green());

    // filtering stage
    let mut filtered_images = vec![];
    let mut skipped_images = vec![];
    for path in images.into_iter() {
        match path.is_file() {
            true => (),
            false => {
                println!(
                    " {} {} {}",
                    ">".bright_yellow().bold(),
                    path.to_string_lossy(),
                    "- not a file".bright_black().italic()
                );
                skipped_images.push(path);
                continue;
            }
        }

        let image = match image::open(&path) {
            Ok(image) => (image.height(), path),
            Err(err) => {
                println!(
                    " {} {} {}",
                    ">".bright_yellow().bold(),
                    path.to_string_lossy(),
                    format!("- {}", err).to_lowercase().bright_black().italic()
                );
                skipped_images.push(path);
                continue;
            }
        };

        filtered_images.push(image);
    }

    // sort by height
    let images = filtered_images
        .into_iter()
        .sorted_by(|a, b| b.0.cmp(&a.0))
        .map(|v| v.1)
        .collect_vec();

    // check if no images were provided
    if images.is_empty() {
        println!(
            " {} no files provided for packing!",
            "error:".bright_red().bold()
        );
        return;
    }

    // packing stage
    let mut failed_images = vec![];
    for path in images {
        match context.pack(&path, gap).is_some() {
            true => println!(
                " {} {} {}",
                "+".bright_green().bold(),
                path.to_string_lossy(),
                "- ok".bright_black().italic()
            ),
            false => {
                println!(
                    " {} {} {}",
                    "!".bright_red().bold(),
                    path.to_string_lossy(),
                    "- not enough space".bright_black().italic()
                );
                failed_images.push(path);
            }
        }
    }

    // print out skipped_images
    if !skipped_images.is_empty() {
        println!();
        println!("{}", "   SKIPPED   ".black().on_bright_yellow());
        for path in skipped_images {
            println!(" {} {}", ">".bold(), path.to_string_lossy());
        }
    }

    // print out failed_images
    if !failed_images.is_empty() {
        println!();
        println!("{}", "   FAILED    ".black().on_bright_red());
        for path in &failed_images {
            println!(" {} {}", ">".bold(), path.to_string_lossy());
        }
    }

    // write to file
    println!();
    println!("{}", "   OUTPUT    ".black().on_bright_white());
    match context.save_to_file(&name, image, dict) {
        Ok(_) => {
            println!(" {} {}.{}", "> image:".bold(), name, image.ext());
            if let Some(dict) = dict {
                println!(" {} {}.{}", "> dictionary:".bold(), name, dict.ext());
            }

            // print out attention if failed_images > 0
            if !failed_images.is_empty() {
                println!();
                println!(
                    " {} {}",
                    "ATTENTION:".red().bold(),
                    format!("{} images failed to pack!", failed_images.len()).red(),
                );
                println!("  {}",
                "Consider increasing the output size using -w and/or -h!"
                    .bright_black()
                    .italic()
                     );
            }
        }
        Err(err) => {
            println!(" {} {}", "error:".bright_red().bold(), err);
        }
    }

    println!();
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
