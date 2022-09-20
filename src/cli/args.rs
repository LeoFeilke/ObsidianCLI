use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Obsidian CLI Tool Arguments
#[derive(Parser, Debug)]
#[clap(name = "Obsidian CLI")]
#[clap(author = "Leonardo Feilke <leofeilke@hotmail.com>")]
#[clap(version = "0.1")]
#[clap(about = "Do awesome things with Obsidian", long_about = None)]
pub struct Cli {
    /// The path to the file to read
    #[clap(short, long, parse(from_os_str))]
    pub file: PathBuf,

    /// The path to the file to directory
    #[clap(short, long, parse(from_os_str))]
    pub path: PathBuf,

    // #[clap(short = 'P', long, parse(from_os_str))]
    // pub path: PathBuf,
    #[clap(short, long, action)]
    pub verbose: bool,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get Obsidian resources
    #[clap(about = "Get Obsidian Resource(s) from file(s)")]
    Get(Resource),
    // Add a resource to existing file/files
    #[clap(about = "Add Obsidian Resource(s) to file(s)")]
    Add(Resource),
}

#[derive(Args, Debug)]
pub struct Resource {
    // Tags. Ex.: "#Science"
    #[clap(short, long, value_parser, help = "Tags. Example: #Science")]
    pub tag: Option<String>,

    // References to other files. Ex.: "[[Einstein]]"
    #[clap(
        short,
        long,
        value_parser,
        help = "References to other files. Example: [[Einstein]]"
    )]
    pub reference: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum ResourceCommands {
    #[clap(about = "Tags")]
    Tag,

    #[clap(about = "References to other files")]
    Reference,
}
