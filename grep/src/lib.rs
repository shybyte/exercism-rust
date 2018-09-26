extern crate failure;
extern crate regex;

use failure::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

#[derive(Debug, Default)]
pub struct Flags {
    print_line_numbers: bool,
    case_insensitive: bool,
    print_file_names: bool,
    match_entire_lines: bool,
    invert: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            print_line_numbers: flags.contains(&"-n"),
            case_insensitive: flags.contains(&"-i"),
            print_file_names: flags.contains(&"-l"),
            match_entire_lines: flags.contains(&"-x"),
            invert: flags.contains(&"-v"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let regexp = make_regexp(pattern, flags)?;

    let mut result = vec![];

    for file_name in files {
        let file = File::open(file_name)?;
        for (line_index, line_result) in BufReader::new(file).lines().enumerate() {
            let mut line = line_result?;
            if regexp.is_match(&line) ^ flags.invert {
                if flags.print_file_names {
                    result.push(file_name.to_string());
                    break;
                }

                if flags.print_line_numbers {
                    line.insert_str(0, &format!("{}:", line_index + 1))
                }

                if files.len() > 1 {
                    line.insert_str(0, &format!("{}:", file_name))
                }

                result.push(line);
            }
        }
    }

    Ok(result)
}

fn make_regexp(pattern_arg: &str, flags: &Flags) -> Result<Regex, Error> {
    let mut pattern = pattern_arg.to_string();

    if flags.case_insensitive {
        pattern.insert_str(0, "(?i)");
    }

    if flags.match_entire_lines {
        pattern = format!("^{}$", pattern);
    }

    Regex::new(&pattern).map_err(Error::from)
}