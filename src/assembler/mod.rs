use crate::instruction::Opcode;
pub mod opcode_parser;
pub mod operand_parser;
pub mod register_parser;

#[derive(Debug,PartialEq)]
pub enum Token{
    Op{code: Opcode},
    Register{reg: u8},
    IntergerOperand{val: i32}
}


