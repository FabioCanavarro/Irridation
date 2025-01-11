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

// Symbol
#[derive(Debug)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
    offset: u32
}

impl Symbol{
    pub fn new(name: String, symbol_type: SymbolType, offset: u32) -> Symbol{
        Symbol{name,symbol_type,offset}
    }
}

// SymbolType
#[derive(Debug)]
pub enum SymbolType {
    Label,
}


// AssemblerPhase
#[derive(Debug, PartialEq, Clone, Default)]
pub enum AssemblerPhase {
    #[default]
    First,
    Second
}


// SymbolTable
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


// Assembler
#[derive(Debug)]
pub struct Assembler{
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable
}

impl Default for Assembler{
    fn default() -> Self {Self::new()}
}

impl Assembler{
    pub fn new () -> Assembler{
        Assembler{
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new()
        }
    }

}
