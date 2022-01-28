#![feature(path_file_prefix)]
mod command;

use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

use command::Command;
use command::Segment;

fn parse_line(line: &str) -> Option<Command> {
    // ignore comments
    let line_trim = if let Some(pos) = line.find("//") {
        &line[..pos].trim()
    } else {
        line.trim()
    };

    // skip empty line
    if line_trim.is_empty() {
        return None;
    }

    // tokenize
    let split = line_trim.split_whitespace().collect::<Vec<_>>();

    // parse command
    Some(match split[0] {
        "add" => Command::Add,
        "sub" => Command::Sub,
        "neg" => Command::Neg,
        "eq" => Command::Eq,
        "gt" => Command::Gt,
        "lt" => Command::Lt,
        "and" => Command::And,
        "or" => Command::Or,
        "not" => Command::Not,
        "push" => Command::Push(Segment::from_str(split[1]), split[2].to_owned()),
        "pop" => Command::Pop(Segment::from_str(split[1]), split[2].to_owned()),
        _ => {
            panic!("unsupported vm op")
        }
    })
}

pub struct ParserContext {
    pub comp_op_counter: isize,
    pub fname: String,
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();

    // in file
    let path = PathBuf::from(&args[1]);
    let fname = path.file_stem().unwrap().to_str().unwrap().into();
    let file = File::open(path.clone())?;
    let reader = BufReader::new(file);

    // out file
    let mut out_path = path;
    out_path.set_extension("asm");
    let mut of = File::create(out_path)?;

    // parse and translate
    let mut ctx = ParserContext {
        comp_op_counter: 0,
        fname,
    };

    for line in reader.lines() {
        let line = line?;
        if let Some(command) = parse_line(&line) {
            writeln!(of, "{}", &command.to_asm(&mut ctx))?;
        }
    }
    Ok(())
}
