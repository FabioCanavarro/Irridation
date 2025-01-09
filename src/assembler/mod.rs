use crate::instruction::Opcode;
pub mod opcode_parser;
pub mod operand_parser;
pub mod register_parser;
pub mod instruction_parser;
pub mod program_parser;
pub mod directive_parsers;
pub mod label_parsers;

#[derive(Debug,PartialEq)]
pub enum Token{
    Op{code: Opcode},
    Register{reg: u8},
    IntergerOperand{val: i32},
    LabelDeclaration{name: String},
    LabelUsage{name: String},
    Directive{name: String}
}


