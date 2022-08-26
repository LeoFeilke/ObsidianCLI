use clap::Parser;
use obsidian::{args::Cli, read_file_lines};
use std::process;

fn main() {
    let args = Cli::parse();

    if let Err(e) = read_file_lines(args) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}
