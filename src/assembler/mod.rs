#![allow(dead_code)]
use crate::instruction::Opcode;
use nom::types::CompleteStr;
use program_parser::{program, Program};
pub mod base_assembler;
pub mod directive_parsers;
pub mod instruction_parser;
pub mod label_parsers;
pub mod opcode_parser;
pub mod operand_parser;
pub mod program_parser;
pub mod register_parser;

// Constants
pub const PIE_HEADER_PREFIX: [u8; 4] = [45, 50, 49, 45];
pub const PIE_HEADER_LENGTH: usize = 64;

// Token
#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg: u8 },
    IntergerOperand { val: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
    IrString { name: String },
}

// Symbol
#[derive(Debug)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
    offset: Option<u32>,
}

impl Symbol {
    pub fn new(name: String, symbol_type: SymbolType) -> Symbol {
        Symbol {
            name,
            symbol_type,
            offset: None,
        }
    }
}

// SymbolType
#[derive(Debug)]
pub enum SymbolType {
    Label,
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
                return Some(i.offset.unwrap());
            }
        }
        None
    }

    pub fn has_symbol(&self, symbol: &str) -> bool {
        for i in &self.symbols {
            if i.name == symbol {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::base_assembler::*;
    use super::*;
    use crate::vm::Vm;

    #[test]
    fn test_create_symbol_table() {
        let mut table = SymbolTable::new();
        table.add_symbols(Symbol {
            name: "test".to_string(),
            symbol_type: SymbolType::Label,
            offset: Some(13),
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
        let program: &str =
            ".data\n.code\nload $2 #10\n load $1 #20\n add $1 $2 $3\ntest: inc $1\n jmpe @test\n";
        let mut assembler: Assembler = Assembler::new();
        let result = assembler.assemble(program);
        assert!(result.is_ok());
        let mut vm: Vm = Vm::new();
        vm.add_bytes(result.unwrap());
        // jmpe @test still doesnt increase byte lenght
        assert_eq!(vm.program.len(), 80);
    }
}
