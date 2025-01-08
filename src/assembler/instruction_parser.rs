use super::opcode_parser::*;
use super::register_parser::*;
use super::operand_parser::*;
use super::Token;
use nom::types::CompleteStr;
use nom::multispace;

#[derive(PartialEq,Debug)]
pub struct AssemblerInstruction{
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction{
    pub fn to_bytes(&self) -> Vec<u8>{
        let mut result = vec![];
        match &self.opcode{
            Token::Op { code } => {
                let byte: u8 = u8::from(*code);  // Explicitly use From<Opcode> for u8
                result.push(byte);
            }
        _ => {println!("Non opcode found in opcode field");std::process::exit(1)}
        }

        for i in [&self.operand1,&self.operand2,&self.operand3]{
            // println!("Operand: {:?}",i);
            if let Some(t) = i{AssemblerInstruction::extract_operand(t, &mut result)}
        }

        // Incase the result is not an array witha  length of 4
        while result.len() < 4{
            result.push(0);
        }
        
        // Debugging
        // println!("opcode {:?}",self.opcode);
        // result.iter().for_each(|x| println!("{}", x));

        // ToDo Doesnt work with CAPITAL OPCODE for some reason
        result
    }

    fn extract_operand(t: &Token, result : &mut Vec<u8>){
        match t{
            Token::Register { reg } => {result.push(*reg)},
            Token::IntergerOperand { val } =>{
                let byte = *val as u16;
                let byte1 = byte as u8;
                let byte2 = (byte >> 8) as u8;
                result.push(byte2);
                result.push(byte1);
            },
            _ => {
                println!("Opcode is found in operand field");
                std::process::exit(1);
            }
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
                opcode: o,
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
            opcode: o,
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
                opcode: o,
                operand1: Some(r1),
                operand2: Some(r2),
                operand3: Some(r3),
            }
        )
    )
);

named!(pub instruction<CompleteStr,AssemblerInstruction>,
    do_parse!(
        ins: alt!(

            instruction_three |
            instruction_one |
            instruction_two
        ) >> 
        (
            ins
        )
    )
);

#[cfg(test)]
mod tests{
    use super::*;
    use crate::instruction::Opcode;
    
    #[test]
    fn test_parse_instruction() {
    let result = instruction_one(CompleteStr("load $0 #100"));
    assert_eq!(result,Ok(
        (
            CompleteStr(""),
            AssemblerInstruction{
                opcode: Token::Op { code: Opcode::LOAD },
                operand1: Some(Token::Register { reg: 0 }),
                operand2: Some(Token::IntergerOperand { val: 100 }),
                operand3: None
            }
        )
    ))
    }
    #[test]
    fn test_single_instruction_to_bytes() {
        let (_, result) = instruction_one(CompleteStr("load $0 #100")).unwrap();
        println!("Opcode: {:?}", result.opcode);  // Added debug print
        
        let code_value = match result.opcode {  // Store intermediate value
            Token::Op { code } => {
                let value = u8::from(code);     // Explicit conversion using From trait
                println!("Converted value: {}", value);  // Added debug print
                value
            },
            _ => {
                println!("Non opcode found in opcode field");
                std::process::exit(1)
            }
        };
        
        assert_eq!(code_value, 0);  // Test against stored value
    }

    #[test]
    fn test_parse_instruction_no_operand() {
        let result = instruction_two(CompleteStr("hlt\n"));
        assert!(result.is_ok());
        let (_,res) = result.unwrap();
        assert_eq!(res,AssemblerInstruction{opcode: Token::Op { code: Opcode::HLT },operand1:None,operand2:None,operand3:None});
    }
}






















