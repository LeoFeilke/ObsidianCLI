use std::error::Error;
use std::fs;

pub fn read_file_lines(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Reading lines: {:?}", config.filename);

    let contents = fs::read_to_string(config.filename)?;
    search_tag(&contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn search_tag<'a>(contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        println!("{:?}", line);
        if is_tag_line(line) {
            println!("{:?}", line)
        }
    }
    vec![]
}
/// Checks if a line contains a Tag.
///
/// A Tag starts with Hashtag and directly continues a string.
fn is_tag_line(line: &str) -> bool {
    if line.starts_with("#") {
        return !line.replace('#', "").starts_with(char::is_whitespace);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_tag() {
        let contents: &str = "#TAG";
        assert!(is_tag_line(&contents));
    }

    #[test]
    fn is_not_tag() {
        let contents: &str = "# TAG";
        assert!(!is_tag_line(&contents));
    }
}
