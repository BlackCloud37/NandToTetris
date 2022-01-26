// Command
use crate::{code, symbol_table::SymbolTable};

#[derive(Debug)]
pub enum Command {
    // A command like `@xxx`
    //  where `xxx` is symbol or decimal number
    A(String), // A(symbol/number)

    // C command like `dest=comp;jump`
    C(Option<String>, String, Option<String>), // C(dest, comp, jump)

    // pseudo-command like `(xxx)`
    //  where `xxx` is the symbol
    L(String), // L(symbol)
}

impl Command {
    pub fn to_code(&self, symbol_table: &mut SymbolTable) -> Option<String> {
        match self {
            Command::A(x) => {
                let addr = if let Ok(v) = x.parse::<u16>() {
                    if v >= 0b1000_0000_0000_0000u16 {
                        panic!("too big const");
                    }
                    v
                } else if let Some(addr) = symbol_table.get_address(x) {
                    // symbol
                    *addr
                } else {
                    // var
                    symbol_table.add_var_entry(x);
                    *symbol_table.get_address(x)?
                };
                Some(format!("0{}", &format!("{:#017b}", addr)[2..]))
            }
            Command::C(dest, comp, jump) => Some(format!(
                "111{}{}{}",
                code::comp(comp),
                code::dest(dest),
                code::jump(jump)
            )),
            Command::L(_) => None,
        }
    }
}
