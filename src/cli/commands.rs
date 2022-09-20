use super::args::{self, Resource};
use regex::Regex;
use std::{
    error::Error,
    fs::{self, DirEntry, ReadDir},
};

#[allow(dead_code)]
pub fn get_files_from_path(args: &args::Cli) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let mut files: Vec<DirEntry> = Vec::new();

    let paths: ReadDir = fs::read_dir(&args.path).unwrap();

    for path in paths {
        if let Ok(entry) = path {
            files.push(entry);
        }
    }
    return Ok(files);
}

pub fn get_file_lines(args: &args::Cli) -> Result<String, Box<dyn Error>> {
    if args.verbose {
        println!(
            "Reading lines searching for resource in file: {:?}",
            args.file
        );
    }
    Ok(fs::read_to_string(&args.file)?)
}

pub fn get_resource<'a>(args: &args::Cli, resource: &Resource) {
    let files = get_files_from_path(&args).unwrap();
    for file in files {
        println!("Directory files: {}", file.file_name().to_str().unwrap());
    }

    if let Some(tag) = &resource.tag {
        return get_tag(&args, tag.to_string());
    }
    if let Some(reference) = &resource.reference {
        return get_reference(&args, reference.to_string());
    }
}

fn contains_resource(line: &str, regex: &Regex) -> bool {
    regex.is_match(line)
}

fn get_tag<'a>(args: &args::Cli, tag: String) -> () {
    let mut lines: Vec<&str> = Vec::new();
    let tag_regex = r"\B(\#[a-zA-Z0-9]+\b)";
    let regex = Regex::new(tag_regex).unwrap();

    if let Ok(contents) = get_file_lines(&args) {
        for line in contents.lines() {
            if contains_resource(line, &regex) {
                lines.push(line);
            }
        }
        if lines.len() > 0 {
            println!("File contains tag: {}", tag);
            println!("Times found: {}", lines.len());
        } else {
            println!("No tags found within file");
        }
    }
}

fn get_reference<'a>(args: &args::Cli, reference: String) -> () {
    let mut lines: Vec<&str> = Vec::new();
    let regex =
        Regex::new(r"(.?)\[\[([^\#|\]\n\r\t]+)(\#([^\#|\]\n\r\t]+))?(\|([^\#|\]\n\r\t]+))?\]\]")
            .unwrap();

    if let Ok(contents) = get_file_lines(&args) {
        for line in contents.lines() {
            if is_reference_line(line) {
                if contains_resource(line, &regex) {
                    lines.push(line);
                }
            }
        }
        if lines.len() > 0 && args.verbose {
            println!("File contains reference: {}", reference);
            println!("Times found: {}", lines.len());
        }
    }
}

/// Checks if a line contains a Tag.
///
/// A Tag starts with Hashtag and directly continues a string.
#[allow(dead_code)]
pub fn is_tag_line(line: &str) -> bool {
    if line.contains("#") {
        return !line.replace("#", "").starts_with(char::is_whitespace);
    }
    false
}

/// Checks if a line contains a Reference.
///
/// A Tag starts with Hashtag and directly continues a string.
pub fn is_reference_line(line: &str) -> bool {
    if line.contains("[[") {
        return !line.replace("[[", "").starts_with(char::is_whitespace);
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
