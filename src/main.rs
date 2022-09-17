use clap::Parser;
use cli::args;
use cli::args::Commands;
use cli::commands::get_resource;
mod cli;

fn main() -> () {
    let args = args::Cli::parse();

    match &args.command {
        Commands::Get(resource) => get_resource(&args, resource),
        Commands::Add(resource) => {
            if args.verbose {
                println!("'Obsidian add' was used, name is: {:?}", resource.tag)
            }
        }
    }
}
