use nom::types::CompleteStr;
use super::instruction_parser::{instruction, AssemblerInstruction};


#[derive(Debug,PartialEq)]
pub struct Program{
    pub instructions: Vec<AssemblerInstruction> 
}

impl Program{
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut program = vec![];
        for instruction in &self.instructions{
            program.append(&mut instruction.to_bytes());
        }
        program
    }
}



named!(pub program<CompleteStr,Program>,
    do_parse!(
        instructions:  many1!(instruction) >>
        (
            Program{
                instructions
            }
        )
    )
);

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_program_parser() {
        let result = program(CompleteStr("load $1 #100"));
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover,CompleteStr(""));
        assert_eq!(1,p.instructions.len());
    }

    #[test]
    fn program_parse_to_bytes() {
        let result = program(CompleteStr("load $0 #100\n"));
        assert!(result.is_ok());
        let (_,program) = result.unwrap();
        let bytecode = program.to_bytes();
        assert_eq!(bytecode.len(),4);
        print!("{:?}",bytecode);
    }
}




