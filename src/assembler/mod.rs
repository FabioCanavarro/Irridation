use std::default;

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


#[derive(Debug, PartialEq, Clone, Default)]
pub enum AssemblerPhase {
    #[default]
    First,
    Second
}


#[derive(Debug)]
pub enum SymbolTable{
    First
}

impl SymbolTable{
    pub fn new() -> SymbolTable{
        SymbolTable::First
    }
}


impl Default for SymbolTable{
    fn default() -> Self {Self::new()}
}

#[derive(Debug)]
pub struct Assembler{
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable
}

impl Assembler{
    pub fn new () -> Assembler{
        Assembler{
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new()
        }
    }
}

impl Default for Assembler{
    fn default() -> Self {Self::new()}
}


