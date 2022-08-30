use std::path::PathBuf;

use clap::Parser;
use colored::Colorize;
use itertools::Itertools;
use yatp_cli::Cli;
use yatp_cli::Context;

fn main() {
    let Cli {
        inputs,
        gap,
        image,
        dict,
        output,
        width,
        height,
    } = Cli::parse();

    // create context
    let mut context = Context::new(yatp_cli::Size::new(width, height));

    // expand inputs into individual file paths
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
    let inputs = inputs.into_iter().flat_map(expand_path).collect_vec();

    println!("{}", "   PACKING    ".black().on_bright_green());

    // filtering stage
    let mut filtered_inputs = vec![];
    let mut skipped_inputs = vec![];
    for path in inputs.into_iter() {
        match path.is_file() {
            true => (),
            false => {
                println!(
                    " {} {} {}",
                    ">".bright_yellow().bold(),
                    path.to_string_lossy(),
                    "- not a file".bright_black().italic()
                );
                skipped_inputs.push(path);
                continue;
            }
        }

        let input = match image::open(&path) {
            Ok(image) => (image.height(), path),
            Err(err) => {
                println!(
                    " {} {} {}",
                    ">".bright_yellow().bold(),
                    path.to_string_lossy(),
                    format!("- {}", err).to_lowercase().bright_black().italic()
                );
                skipped_inputs.push(path);
                continue;
            }
        };

        filtered_inputs.push(input);
    }

    // sort by height
    let inputs = filtered_inputs
        .into_iter()
        .sorted_by(|a, b| b.0.cmp(&a.0))
        .map(|v| v.1)
        .collect_vec();

    // check if no images were provided
    if inputs.is_empty() {
        println!(
            " {} no files provided for packing!",
            "error:".bright_red().bold()
        );
        return;
    }

    // packing stage
    let mut failed_inputs = vec![];
    for path in inputs {
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
                failed_inputs.push(path);
            }
        }
    }

    // print out skipped_images
    if !skipped_inputs.is_empty() {
        println!();
        println!("{}", "   SKIPPED   ".black().on_bright_yellow());
        for path in skipped_inputs {
            println!(" {} {}", ">".bold(), path.to_string_lossy());
        }
    }

    // print out failed_images
    if !failed_inputs.is_empty() {
        println!();
        println!("{}", "   FAILED    ".black().on_bright_red());
        for path in &failed_inputs {
            println!(" {} {}", ">".bold(), path.to_string_lossy());
        }
    }

    // write to file
    println!();
    println!("{}", "   OUTPUT    ".black().on_bright_white());
    match context.save_to_file(&output, image, dict) {
        Ok(_) => {
            println!(" {} {}.{}", "> image:".bold(), output, image.ext());
            if let Some(dict) = dict {
                println!(" {} {}.{}", "> dictionary:".bold(), output, dict.ext());
            }

            // print out attention if failed_images > 0
            if !failed_inputs.is_empty() {
                println!();
                println!(
                    " {} {}",
                    "ATTENTION:".red().bold(),
                    format!("{} images failed to pack!", failed_inputs.len()).red(),
                );
                println!(
                    "  {}",
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
