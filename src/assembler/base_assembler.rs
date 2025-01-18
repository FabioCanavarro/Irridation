use super::*;

// AssemblerPhase
#[derive(Debug, PartialEq, Clone, Default)]
pub enum AssemblerPhase {
    #[default]
    First,
    Second,
}

#[derive(Debug)]
pub enum AssemblerSection{
    S
}


#[derive(Debug,Clone)]
pub enum AssemblerError{
    InsufficientSections,
    ParseError{errors: String}
}

// Assembler
#[derive(Debug)]
pub struct Assembler {
    /// Tracks which phase the assember is in
    phase: AssemblerPhase,
    /// Symbol table for constants and variables
    pub symbol_table: SymbolTable,
    /// The read-only data section constants are put in
    pub ro: Vec<u8>,
    /// The compiled bytecode generated from the assembly instructions
    pub bytecode: Vec<u8>,
    /// Tracks the current offset of the read-only section
    ro_offset: u32,
    /// A list of all the sections we've seen in the code
    sections: Vec<AssemblerSection>,
    /// The current section the assembler is in
    current_section: Option<AssemblerSection>,
    /// The current instruction the assembler is converting to bytecode
    current_instruction: u32,
    /// Any errors we find along the way. At the end, we'll present them to the user.
    errors: Vec<AssemblerError>
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
            ro: vec![],
            bytecode: vec![],
            ro_offset: 0,
            sections: vec![],
            current_section: None,
            current_instruction: 0,
            errors: vec![]
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
        // First pass
        self.extract_label(p);
        self.phase = AssemblerPhase::Second;
    }

    pub fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        // Second pass
        let mut program = vec![];
        for i in &p.instructions {
            let mut bytes = i.to_bytes(&self.symbol_table);
            program.append(&mut bytes);
        }
        program
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

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        match program(CompleteStr(raw)) {
            Ok((_, p)) => {
                let mut result = self.write_pie_header();

                self.process_first_phase(&p);

                if !self.errors.is_empty(){
                    return Err(self.errors.clone());
                }
                result.append(&mut self.process_second_phase(&p));
                Ok(result)
            }
            Err(e) => {
                println!("There wan error parsing the code: {:?}",e);
                Err(vec![AssemblerError::ParseError{errors: e.to_string()}])
            },
        }
    }

    
}
