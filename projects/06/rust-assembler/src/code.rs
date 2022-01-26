/* Translates Hack assembly language mnemonics into binary codes. */
use core::panic;

// three bits code
pub fn dest(x: &Option<String>) -> String {
    match x {
        Some(x) => match &x[..] {
            "M" => "001",
            "D" => "010",
            "MD" | "DM" => "011",
            "A" => "100",
            "AM" | "MA" => "101",
            "AD" | "DA" => "110",
            "AMD" | "ADM" | "DMA" | "DAM" | "MAD" | "MDA" => "111",
            x => panic!("invalid dest string: {}", x),
        },
        None => "000",
    }
    .to_owned()
}

// seven bits code
pub fn comp(x: &str) -> String {
    match x {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" | "1+D" => "0011111",
        "A+1" | "1+A" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" | "A+D" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" | "A&D" => "0000000",
        "D|A" | "A|D" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" | "1+M" => "1110111",
        "M-1" => "1110010",
        "D+M" | "M+D" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" | "M&D" => "1000000",
        "D|M" | "M|D" => "1010101",
        x => panic!("invalid dest string: {}", x),
    }
    .to_owned()
}

// three bits code
pub fn jump(x: &Option<String>) -> String {
    match x {
        Some(x) => match &x[..] {
            "JGT" => "001",
            "JEQ" => "010",
            "JGE" => "011",
            "JLT" => "100",
            "JNE" => "101",
            "JLE" => "110",
            "JMP" => "111",
            x => panic!("invalid jump string: {}", x),
        },
        None => "000",
    }
    .to_owned()
}
