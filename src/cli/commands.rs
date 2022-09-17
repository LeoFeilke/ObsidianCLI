use super::args::{self, Resource};
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
    if args.verbose {
        println!("'Obsidian get' was used, resources are: {:?}", resource)
    }
    if let Some(tag) = &resource.tag {
        println!("'Getting tag: {:?}", resource.tag);
        return get_tag(&args, tag.to_string());
    } else if let Some(reference) = &resource.reference {
        println!("'Getting resource: {:?}", resource.tag);
        return get_reference(&args, reference.to_string());
    }
}

fn get_tag<'a>(args: &args::Cli, tag: String) -> () {
    let mut lines: Vec<&str> = Vec::new();

    if let Ok(contents) = get_file_lines(&args) {
        for line in contents.lines() {
            if is_tag_line(line) {
                let tag_with_hashtag = format!("#{}", tag);
                if line.contains(&tag_with_hashtag) {
                    lines.push(line);
                }
            }
        }
        if lines.len() > 0 {
            println!("File contains tag: {}", tag);
            println!("Times found: {}", lines.len());
        }
    }
}

fn get_reference<'a>(args: &args::Cli, reference: String) -> () {
    let mut lines: Vec<&str> = Vec::new();

    if let Ok(contents) = get_file_lines(&args) {
        for line in contents.lines() {
            if is_reference_line(line) {
                let refence_with_format = format!("[[{}]]", reference);
                if line.contains(&refence_with_format) {
                    lines.push(line);
                    println!("File contains reference: {}", line)
                }
            }
        }
        println!("Times found: {}", lines.len());
    }
}

/// Checks if a line contains a Tag.
///
/// A Tag starts with Hashtag and directly continues a string.
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
