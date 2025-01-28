use instruction_parser::AssemblerInstruction;

use super::*;

// AssemblerPhase
#[derive(Debug, PartialEq, Clone, Default)]
pub enum AssemblerPhase {
    #[default]
    First,
    Second,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerSection {
    Data { starting_instruction: Option<u32> },
    Code { starting_instruction: Option<u32> },
    Unknown,
}

impl<'a> From<&'a str> for AssemblerSection {
    fn from(name: &str) -> AssemblerSection {
        match name {
            "data" => AssemblerSection::Data {
                starting_instruction: None,
            },
            "code" => AssemblerSection::Code {
                starting_instruction: None,
            },
            _ => AssemblerSection::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AssemblerError {
    NoSegmentDeclarationFound { instruction: u32 },
    StringConstantDeclaredWithoutLabel { instruction: u32 },
    SymbolAlreadyDeclared,
    UnknownDirectiveFound { directive: String },
    NonOpcodeInOpcodeField,
    InsufficientSections,
    ParseError { error: String },
}

// Assembler
#[derive(Debug)]
pub struct Assembler {
    phase: AssemblerPhase,

    pub symbol_table: SymbolTable,

    pub ro: Vec<u8>,

    pub bytecode: Vec<u8>,

    ro_offset: u32,

    pub sections: Vec<AssemblerSection>,

    current_section: Option<AssemblerSection>,

    current_instruction: u32,

    // Collect all errors to present to the user in the end.
    errors: Vec<AssemblerError>,
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
            errors: vec![],
        }
    }

    pub fn extract_label(&mut self, p: &Program) {
        let mut c = 0;
        for i in &p.instructions {
            if i.is_label() {
                if let Some(name) = i.get_label_name() {
                    self.symbol_table.symbols.push(Symbol {
                        name,
                        symbol_type: SymbolType::Label,
                        offset: Some(c),
                    })
                }
            }
            c += 4;
        }
    }

    fn process_label_declaration(&mut self, i: &AssemblerInstruction) {
        let name = match i.get_label_name() {
            Some(name) => name,
            None => {
                self.errors
                    .push(AssemblerError::StringConstantDeclaredWithoutLabel {
                        instruction: self.current_instruction,
                    });
                return;
            }
        };

        if self.symbol_table.has_symbol(&name) {
            self.errors.push(AssemblerError::SymbolAlreadyDeclared);
            return;
        }

        self.symbol_table
            .add_symbols(Symbol::new(name, SymbolType::Label))
    }

    pub fn process_first_phase(&mut self, p: &Program) {
        // First pass
        for i in &p.instructions {
            if i.is_label() {
                if self.current_section.is_some() {
                    self.process_label_declaration(i);
                } else {
                    // If we have *not* hit a segment header yet, then we have a label outside of a segment, which is not allowed
                    self.errors.push(AssemblerError::NoSegmentDeclarationFound {
                        instruction: self.current_instruction,
                    });
                }
            }
            if i.is_directive() {
                self.process_directive(i);
            }

            self.current_instruction += 1;
        }
        self.phase = AssemblerPhase::Second;
    }

    pub fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        self.current_instruction = 0;
        // Second pass
        let mut program = vec![];
        for i in &p.instructions {
            if i.is_opcode() {
                let mut bytes = i.to_bytes(&self.symbol_table);
                program.append(&mut bytes);
            }
            if i.is_directive() {
                self.process_directive(i);
            }
            self.current_instruction += 1
        }
        program
    }
    fn process_directive(&mut self, i: &AssemblerInstruction) {
        let directive_name = match i.get_directive_name() {
            Some(name) => name,
            None => {
                println!("Directive has an invalid name: {:?}", i);
                return;
            }
        };
        if i.has_operand() {
            match directive_name.as_ref() {
                "asciiz" => {
                    // handle_asciiz func
                    todo!()
                }
                _ => {
                    self.errors.push(AssemblerError::UnknownDirectiveFound {
                        directive: directive_name.clone(),
                    });
                }
            }
        } else {
            self.process_section_header(&directive_name);
        }
    }

    fn process_section_header(&mut self, header_name: &str) {
        let new_section: AssemblerSection = header_name.into();

        if new_section == AssemblerSection::Unknown {
            println!(
                "Found an section header that is unknown: {:#?}",
                header_name
            );
            return;
        }
        self.sections.push(new_section.clone());
        self.current_section = Some(new_section);
    }

    fn handle_asciiz(&mut self, i: &AssemblerInstruction) {
        if self.phase != AssemblerPhase::First{return;}

        match i.get_string_constant(){
            Some(string) => {
                match i.get_label_name(){
                    Some(name) => self.symbol_table.set_symbol_offset(&name, self.ro_offset),
                    None => {
                        println!("Found a String constant with no associated label was found");
                        return;
                    }
                };

                for byte in string.as_bytes(){
                    self.ro.push(*byte);
                    self.ro_offset+=1
                }
                self.ro.push(0);
                self.ro_offset+=1;
            },
            _ => {
                println!("No String Constant founded");
            }
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

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        match program(CompleteStr(raw)) {
            Ok((_, p)) => {
                let mut result = self.write_pie_header();

                self.process_first_phase(&p);

                if !self.errors.is_empty() {
                    return Err(self.errors.clone());
                }
                if self.sections.len() != 2 {
                    print!("found {} sections, needed 2", self.sections.len());
                    self.errors.push(AssemblerError::InsufficientSections);
                    return Err(self.errors.clone());
                }

                result.append(&mut self.process_second_phase(&p));
                Ok(result)
            }
            Err(e) => {
                println!("There was an error parsing the code: {:?}", e);
                Err(vec![AssemblerError::ParseError {
                    error: e.to_string(),
                }])
            }
        }
    }
}
