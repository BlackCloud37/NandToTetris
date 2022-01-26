use std::{env, fs::File, io::Write, path::PathBuf};

mod code;
mod command;
mod parser;
mod symbol_table;

// if the input program is not valid, the assembler will panic
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("invalid usage")
    }

    let path = PathBuf::from(&args[1]);
    // first pass
    let mut symbol_table = symbol_table::SymbolTable::new();
    let mut addr = 0;
    for command in parser::Parser::open(&path)? {
        match command {
            command::Command::L(symbol) => {
                symbol_table.add_entry(&symbol, addr);
            }
            _ => addr += 1,
        }
    }

    // second pass
    let mut output = path.clone();
    output.set_extension("hack");
    let mut of = File::create(output)?;
    for command in parser::Parser::open(&path)? {
        if let Some(code) = command.to_code(&mut symbol_table) {
            write!(of, "{}\n", code)?;
        }
    }
    Ok(())
}
