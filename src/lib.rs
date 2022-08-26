use std::error::Error;
use std::fs;
pub mod args;

pub fn read_file_lines(args: args::Cli) -> Result<(), Box<dyn Error>> {
    println!("Reading lines searching for Tag: {:?}", args.pattern);

    let contents = fs::read_to_string(args.path)?;
    search_tag(&contents, args.pattern);
    Ok(())
}

fn search_tag<'a>(contents: &'a str, tag: String) -> Vec<&'a str> {
    for line in contents.lines() {
        // let split = line.split(" ");
        // for s in split {
        //     println!("{}", s)
        // }

        if is_tag_line(line) {
            let tag_with_hashtag = format!("#{}", tag);
            if line.contains(&tag_with_hashtag) {
                println!("File contains tag {}", line)
            }
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
