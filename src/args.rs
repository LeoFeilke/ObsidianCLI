use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Obsidian CLI Tool Arguments
#[derive(Parser)]
#[clap(name = "Obsidian CLI")]
#[clap(author = "Leonardo Feilke <leofeilke@hotmail.com>")]
#[clap(version = "0.1")]
#[clap(about = "Does awesome things with Obsidian through the CLI", long_about = None)]
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,

    /// The path to the file to read
    #[clap(parse(from_os_str))]
    pub path: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[clap(short, long, action)]
        list: bool,
    },
}
