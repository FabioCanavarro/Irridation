use super::directive_parsers::*;
use super::label_parsers::*;
use super::opcode_parser::*;
use super::operand_parser::*;
use super::register_parser::*;
use super::SymbolTable;
use super::Token;
use nom::multispace;
use nom::types::CompleteStr;

#[derive(PartialEq, Debug)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self, symbol_table: &SymbolTable) -> Vec<u8> {
        let mut result = vec![];
        match &self.opcode {
            Some(Token::Op { code }) => {
                let byte: u8 = u8::from(*code); // Explicitly use From<Opcode> for u8
                result.push(byte);
            }
            _ => {
                println!("Non opcode found in opcode field");
                std::process::exit(1)
            }
        }

        for i in [&self.operand1, &self.operand2, &self.operand3] {
            // println!("Operand: {:?}",i);
            if let Some(t) = i {
                AssemblerInstruction::extract_operand(t, &mut result)
            }
        }

        // Incase the result is not an array witha  length of 4
        while result.len() < 4 {
            result.push(0);
        }

        // Debugging
        // println!("opcode {:?}",self.opcode);
        // result.iter().for_each(|x| println!("{}", x));

        // ToDo Doesnt work with CAPITAL OPCODE for some reason
        result
    }

    fn extract_operand(t: &Token, result: &mut Vec<u8>) {
        match t {
            Token::Register { reg } => result.push(*reg),
            Token::IntergerOperand { val } => {
                let byte = *val as u16;
                let byte1 = byte as u8;
                let byte2 = (byte >> 8) as u8;
                result.push(byte2);
                result.push(byte1);
            }
            _ => {
                println!("Opcode is found in operand field");
                std::process::exit(1);
            }
        }
    }

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn is_opcode(&self) -> bool {
        self.opcode.is_some()
    }

    pub fn is_directive(&self) -> bool {
        self.directive.is_some()
    }

    pub fn label_name(&self) -> Option<String> {
        match &self.label {
            Some(l) => match l {
                Token::LabelDeclaration { name } => Some(name.clone()),
                _ => None,
            },
            None => None,
        }
    }
}

named!(instruction_one<CompleteStr,AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r: register >>
        i: interger_operand >>
        (
            AssemblerInstruction{
                label: None,
                directive: None,
                opcode: Some(o),
                operand1: Some(r),
                operand2: Some(i),
                operand3: None
            }
        )
    )
);

named!(instruction_two<CompleteStr,AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
            label: None,
            directive: None,
            opcode: Some(o),
            operand1: None,
            operand2: None,
            operand3: None
            }
    )
));

named!(instruction_three<CompleteStr,AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r1: register >>
        r2: register >>
        r3: register >>
        (
            AssemblerInstruction{
                label: None,
                directive: None,
                opcode: Some(o),
                operand1: Some(r1),
                operand2: Some(r2),
                operand3: Some(r3),
            }
        )
    )
);

named!(instruction_four<CompleteStr,AssemblerInstruction>,
    do_parse!(
        l: opt!(label_declaration) >>
        o: opcode >>
        o1: opt!(operand)>>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                label: l,
                directive: None,
                operand1: o1,
                operand2: o2,
                operand3: o3
            }
        )
    )
);
named!(pub instruction_combined<CompleteStr,AssemblerInstruction>,
    do_parse!(
        ins: alt!(

            instruction_three |
            instruction_one |
            instruction_two |
            instruction_four
        ) >>
        (
            ins
        )
    )
);

// Will try to parse out any of the Instruction forms
named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_combined |
            directive
        ) >>
        (
            ins
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction() {
        let result = instruction_one(CompleteStr("load $0 #100"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    label: None,
                    directive: None,
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    operand1: Some(Token::Register { reg: 0 }),
                    operand2: Some(Token::IntergerOperand { val: 100 }),
                    operand3: None
                }
            ))
        )
    }
    #[test]
    fn test_single_instruction_to_bytes() {
        let (_, result) = instruction_one(CompleteStr("load $0 #100")).unwrap();
        println!("Opcode: {:?}", result.opcode); // Added debug print

        let code_value = match result.opcode {
            // Store intermediate value
            Some(Token::Op { code }) => {
                let value = u8::from(code); // Explicit conversion using From trait
                println!("Converted value: {}", value); // Added debug print
                value
            }
            _ => {
                println!("Non opcode found in opcode field");
                std::process::exit(1)
            }
        };

        assert_eq!(code_value, 0); // Test against stored value
    }

    #[test]
    fn test_parse_instruction_no_operand() {
        let result = instruction_two(CompleteStr("hlt\n"));
        assert!(result.is_ok());
        let (_, res) = result.unwrap();
        assert_eq!(
            res,
            AssemblerInstruction {
                opcode: Some(Token::Op { code: Opcode::HLT }),
                label: None,
                directive: None,
                operand1: None,
                operand2: None,
                operand3: None
            }
        );
    }
}
