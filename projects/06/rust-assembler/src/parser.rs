/* Encapsulates access to the input code.
 * Reads an assembly language com- mand, parses it,
 * and provides convenient access to the command’s components (fields and symbols).
 * In addition, removes all white space and comments.
*/

// Ref: https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust

use crate::command::Command;
use std::{
    fs::File,
    io::{self, prelude::*},
};

// Parser
pub struct Parser {
    reader: io::BufReader<File>,
}

impl Parser {
    pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);
        Ok(Self { reader })
    }
}

// as for empty/space line, returns None
//    for valid line, returns command
//    for invalid but not empty line, panic
fn parse_line(line: &str) -> Option<Command> {
    let line_trim = if let Some(pos) = line.find("//") {
        &line[..pos].trim()
    } else {
        line.trim()
    };

    // let line_trim = line.trim();
    let len = line_trim.len();

    // empty line
    if line_trim.is_empty() {
        return None;
    }

    // A: `@xxx`
    if line_trim.starts_with("@") {
        if len == 1 {
            panic!("invalid A-command")
        }

        return Some(Command::A(line_trim[1..].to_owned())); // remove `@`
    }

    // L
    if line_trim.starts_with("(") && line_trim.ends_with(")") {
        if len == 2 {
            panic!("invalid L-command")
        }
        return Some(Command::L(line_trim[1..len - 1].to_owned())); // remove `()`
    }

    // C
    let split = line_trim.split(&['=', ';', ' ']).collect::<Vec<_>>();

    Some(match split.len() {
        3 => Command::C(
            Some(split[0].to_owned()),
            split[1].to_owned(),
            Some(split[2].to_owned()),
        ),
        2 if line_trim.find("=").is_some() => {
            Command::C(Some(split[0].to_owned()), split[1].to_owned(), None)
        }
        2 if line_trim.find(";").is_some() => {
            Command::C(None, split[0].to_owned(), Some(split[1].to_owned()))
        }
        1 => Command::C(None, String::new(), None),
        _ => {
            panic!("invalid C-command, {}", &line_trim);
        }
    })
}

impl Iterator for Parser {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::with_capacity(1024);

        while let Ok(x) = self.reader.read_line(&mut buf) {
            if x > 0 {
                if let Some(command) = parse_line(&buf) {
                    return Some(command);
                } else {
                    buf.clear();
                    continue;
                }
            } else {
                // EOF
                return None;
            }
        }
        // I/O error
        None
    }
}
