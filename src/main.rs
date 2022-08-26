use obsidian::read_file_lines;
use obsidian::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {:?}", err);

        process::exit(1);
    });

    if let Err(e) = read_file_lines(config) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}
