use nom::types::CompleteStr;
use super::instruction_parser::{AssemblerInstruction,instruction_one};

#[derive(Debug,PartialEq)]
pub struct Program{
    instructions: Vec<AssemblerInstruction>
}

named!(pub program<CompleteStr,Program>,
    do_parse!(
        instructions:  many1!(instruction_one) >>
        (
            Program{
                instructions
            }
        )
    )
);

mod tests{
    use super::*;

    #[test]
    fn test_program_parser() {
        let result = program(CompleteStr("load $1 #100"));
        let (leftover, p) = result.unwrap();
        assert_eq!(leftover,CompleteStr(""));
        assert_eq!(1,p.instructions.len());
    }
}




