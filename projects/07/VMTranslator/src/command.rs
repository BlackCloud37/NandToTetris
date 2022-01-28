use indoc::indoc;

use Command::*;
use Segment::*;

#[derive(Debug)]
pub enum Segment {
    Constant,
    Local,
    Argument,
    This,
    That,
    Temp,
    Pointer,
    Static,
}

impl Segment {
    pub fn from_str(name: &str) -> Self {
        match name {
            "constant" => Constant,
            "local" => Local,
            "argument" => Argument,
            "this" => This,
            "that" => That,
            "temp" => Temp,
            "pointer" => Pointer,
            "static" => Static,
            _ => {
                panic!("unsupported push type")
            }
        }
    }

    pub fn get_symbol(&self) -> String {
        match self {
            Local => "LCL",
            Argument => "ARG",
            This => "THIS",
            That => "THAT",
            Temp => "5",
            Pointer => "3",
            _ => {
                panic!("no symbol")
            }
        }
        .to_owned()
    }
}

#[derive(Debug)]
pub enum Command {
    Push(Segment, String),
    Pop(Segment, String),

    Add,
    Sub,
    And,
    Or,

    Gt,
    Lt,
    Eq,

    Neg,
    Not,
}

impl Command {
    fn get_operaotr(&self) -> String {
        match self {
            Add => "+",
            Sub => "-",
            And => "&",
            Or => "|",
            Neg => "-",
            Not => "!",
            Gt => "JLE",
            Lt => "JGE",
            Eq => "JNE",
            _ => {
                panic!("non operator")
            }
        }
        .to_owned()
    }

    pub fn to_asm(&self, ctx: &mut crate::ParserContext) -> String {
        let asm_code = match self {
            Push(seg, s) => {
                let push_code = match seg {
                    Constant => format!(
                        indoc! {"
                            @{}
                            D=A
                        "},
                        s
                    ),
                    Local | Argument | This | That => format!(
                        indoc! {"
                            @{}
                            D=A
                            @{}
                            A=M+D
                            D=M
                        "},
                        s,
                        seg.get_symbol()
                    ),
                    Temp | Pointer => format!(
                        indoc! {"
                            @{}
                            D=A
                            @{}
                            A=A+D
                            D=M
                        "},
                        s,
                        seg.get_symbol()
                    ),
                    Static => format!(
                        indoc! {"
                            @{}
                            D=M
                        "},
                        format!("{}.{}", ctx.fname, s)
                    ),
                };

                format!(
                    indoc! {"
                        {}
                        @SP    // set stack top
                        A=M
                        M=D
                        @SP
                        M=M+1  // sp++
                    "},
                    push_code
                )
            }
            Pop(seg, s) => {
                let pop_code = match seg {
                    Local | Argument | This | That => format!(
                        indoc! {"
                            @{}
                            D=A
                            @{}
                            D=M+D
                            @R15
                            M=D
                        "},
                        s,
                        seg.get_symbol(),
                    ),
                    Temp | Pointer => format!(
                        indoc! {"
                            @{}
                            D=A
                            @{}
                            D=A+D
                            @R15
                            M=D
                        "},
                        s,
                        seg.get_symbol()
                    ),
                    Static => format!(
                        indoc! {"
                            @{}
                            D=M
                            @R15
                            A=M
                            M=D
                        "},
                        format!("{}.{}", ctx.fname, s)
                    ),
                    Constant => panic!("pop constant"),
                };

                format!(
                    indoc! {"
                        {}
                        @SP
                        AM=M-1
                        D=M
                        @R15
                        A=M
                        M=D
                    "},
                    pop_code
                )
            }
            Add | Sub | And | Or => format!(
                indoc! {"
                    @SP
                    AM=M-1    // sp--
                    D=M       // D = *sp
                    @SP
                    A=M-1     // A now points to the first operand
                    M=M{}D     // M = *sp op D, replace the first operand localy
                "},
                self.get_operaotr()
            ),
            Not | Neg => format!(
                indoc! {"
                    @SP
                    A=M-1     // A now points to the operand
                    M={}M
                "},
                self.get_operaotr()
            ),
            Gt | Lt | Eq => {
                ctx.comp_op_counter += 1;
                format!(
                    indoc! {"
                        @SP
                        AM=M-1
                        D=M
                        A=A-1
                        D=M-D     // D = op1 - op2
                        M=0       // assert false
                        @{0}
                        D;{1}
                        @SP
                        A=M-1
                        M=-1
                        ({0})
                    "},
                    format!("comp.{}", ctx.comp_op_counter),
                    self.get_operaotr()
                )
            }
        };

        format!("// {:?}\n{}", self, asm_code)
    }
}