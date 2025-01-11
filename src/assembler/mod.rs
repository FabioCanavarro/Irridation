use program_parser::Program;

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
pub struct SymbolTable{
    symbols: Vec<Symbol>
}

impl Default for SymbolTable{
    fn default() -> Self{
        Self::new()
    }
}

impl SymbolTable{
    pub fn new() -> SymbolTable{
        SymbolTable{
            symbols: vec![]
        }
    }

    pub fn add_symbols(&mut self, symbol: Symbol){
        self.symbols.push(symbol);
    }
    
    pub fn symbol_value(&self, symbol: Symbol) -> Option<u32>{
        for i in &self.symbols{
            if i.name == symbol.name{
                return Some(symbol.offset)
            }
        }
        None
    }

}


// Assembler
#[derive(Debug)]
pub struct Assembler{
    pub phase: AssemblerPhase,
    pub symbol_table: SymbolTable
}

impl Default for Assembler{
    fn default() -> Self {Self::new()}
}

impl Assembler{
    pub fn new () -> Assembler{
        Assembler{
            phase: AssemblerPhase::First,
            symbol_table: SymbolTable::new()
        }
    }
    
    pub fn extract_label(&mut self, p: &Program){
        let mut c = 0;
        for i in &p.instructions{
            if i.is_label(){
                if let Some(name) = i.label_name(){
                    self.symbol_table.symbols.push(Symbol { name, symbol_type: SymbolType::Label, offset: c})
                }
            }
            c+=4;
        }
    }

    pub fn process_first_phase(&mut self, p: &Program){
        self.extract_label(p);
        self.phase = AssemblerPhase::Second;
    }

    pub fn process_second_phase(&mut self, p: &Program) -> Vec<u8>{
        let mut program = vec![];
        for i in &p.instructions{
            let mut bytes = i.to_bytes(&self.symbol_table);
            program.append(&mut bytes);
        }
        program
    }

}



















