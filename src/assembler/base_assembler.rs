use super::*;

// AssemblerPhase
#[derive(Debug, PartialEq, Clone, Default)]
pub enum AssemblerPhase {
    #[default]
    First,
    Second,
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
                let mut result = self.write_pie_header();

                self.process_first_phase(&p);
                result.append(&mut self.process_second_phase(&p));
                Some(result)
            }
            Err(_) => None,
        }
    }

    fn write_pie_header(&self) -> Vec<u8> {
        let mut header = vec![];
        for byte in PIE_HEADER_PREFIX {
            header.push(byte);
        }

        while header.len() < PIE_HEADER_LENGTH {
            header.push(0);
        }
        header
    }
}
