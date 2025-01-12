use crate::instruction::Opcode;
use nom::types::CompleteStr;
use program_parser::{program, Program};
pub mod directive_parsers;
pub mod instruction_parser;
pub mod label_parsers;
pub mod opcode_parser;
pub mod operand_parser;
pub mod program_parser;
pub mod register_parser;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg: u8 },
    IntergerOperand { val: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}

// Symbol
#[derive(Debug)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
    offset: u32,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
        Symbol {
            name,
            symbol_type,
            offset,
        }
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
    Second,
}

// SymbolTable
#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { symbols: vec![] }
    }

    pub fn add_symbols(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn symbol_value(&self, symbol: &str) -> Option<u32> {
        for i in &self.symbols {
            if i.name == symbol {
                return Some(i.offset);
            }
        }
        None
    }
}

// Assembler
#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbol_table: SymbolTable,
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbol_table: SymbolTable::new(),
        }
    }

    pub fn extract_label(&mut self, p: &Program) {
        let mut c = 0;
        for i in &p.instructions {
            if i.is_label() {
                if let Some(name) = i.label_name() {
                    self.symbol_table.symbols.push(Symbol {
                        name,
                        symbol_type: SymbolType::Label,
                        offset: c,
                    })
                }
            }
            c += 4;
        }
    }

    pub fn process_first_phase(&mut self, p: &Program) {
        self.extract_label(p);
        self.phase = AssemblerPhase::Second;
    }

    pub fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        let mut program = vec![];
        for i in &p.instructions {
            let mut bytes = i.to_bytes(&self.symbol_table);
            program.append(&mut bytes);
        }
        program
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program(CompleteStr(raw)) {
            Ok((_, p)) => {
                self.process_first_phase(&p);
                Some(self.process_second_phase(&p))
            }
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::Vm;

    #[test]
    fn test_create_symbol_table() {
        let mut table = SymbolTable::new();
        table.add_symbols(Symbol {
            name: "test".to_string(),
            symbol_type: SymbolType::Label,
            offset: 13,
        });
        assert_eq!(table.symbols.len(), 1);
        let symbol_val = table.symbol_value("test");
        assert!(symbol_val.is_some());
        assert_eq!(symbol_val.unwrap(), 13);
        let symbol_val = table.symbol_value("error");
        assert!(symbol_val.is_none());
    }

    #[test]
    fn test_assemble_program() {
        let program: &str = "load $2 #10\n load $1 #20\n add $1 $2 $3\ntest: inc $1\n jmpe @test\n";
        let mut assembler: Assembler = Assembler::new();
        let result = assembler.assemble(program);
        assert!(result.is_some());
        let mut vm: Vm = Vm::new();
        vm.add_bytes(result.unwrap());
        // jmpe @test still doesnt increase byte lenght
        assert_eq!(vm.program.len(), 16);
    }
}
