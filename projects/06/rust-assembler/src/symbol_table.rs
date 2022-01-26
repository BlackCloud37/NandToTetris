/* Keeps a correspondence between symbolic labels and numeric addresses. */

use std::collections::HashMap;
pub struct SymbolTable {
    entries: HashMap<String, u16>,
    var_addr: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            entries: [
                ("SP", 0),
                ("LCL", 1),
                ("ARG", 2),
                ("THIS", 3),
                ("THAT", 4),
                ("R0", 0),
                ("R1", 1),
                ("R2", 2),
                ("R3", 3),
                ("R4", 4),
                ("R5", 5),
                ("R6", 6),
                ("R7", 7),
                ("R8", 8),
                ("R9", 9),
                ("R10", 10),
                ("R11", 11),
                ("R12", 12),
                ("R13", 13),
                ("R14", 14),
                ("R15", 15),
                ("SCREEN", 16384),
                ("KBD", 24576),
            ]
            .map(|(k, v)| (k.to_owned(), v))
            .into_iter()
            .collect(),
            var_addr: 0x0010,
        }
    }

    pub fn add_entry(&mut self, symbol: &str, addr: u16) {
        self.entries.insert(symbol.to_owned(), addr);
    }

    pub fn add_var_entry(&mut self, symbol: &str) {
        self.entries.insert(symbol.to_owned(), self.var_addr);
        self.var_addr += 1;
    }

    pub fn get_address(&self, symbol: &str) -> Option<&u16> {
        self.entries.get(symbol)
    }
}
