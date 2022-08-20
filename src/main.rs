use clap::Parser;
use cli::Cli;
use context::Context;

mod cli;
mod context;
mod dictionary;
mod format;

fn main() {
    let cli = Cli::parse();
    let mut context = Context::new();

    let input = cli.inputs;
    let gap = cli.gap;
    let name = cli.output;
    let image = cli.image;
    let dict = cli.dict;

    for file in input
        .into_iter()
        .flat_map(|v| match v.is_dir() {
            true => v
                .read_dir()
                .map(|v| {
                    v.into_iter()
                        .flat_map(|v| v.map(|v| v.path()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            false => vec![v],
        })
        .filter(|v| v.is_file())
    {
        context.pack(&file, gap);
        println!("packed: {}", file.to_string_lossy());
    }

    context.save_to_file(&name, image, dict).unwrap();
}
