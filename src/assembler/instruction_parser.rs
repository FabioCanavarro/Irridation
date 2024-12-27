use super::opcode_parser::*;
use super::register_parser::*;
use super::operand_parser::*;
use super::Token;
use nom::types::CompleteStr;

#[derive(PartialEq,Debug)]
pub struct AssemblerInstruction{
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

named!(pub instruction_one<CompleteStr,AssemblerInstruction>,
    do_parse!(
        o: opcode_load >>
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

mod tests{
    use super::*;
    use crate::instruction::Opcode;

    use super::instruction_one;
    
    #[test]
    fn test_parse_instruction() {
    let result = instruction_one(CompleteStr("load $0 #100"));
    assert_eq!(result,Ok((
        CompleteStr(""),
        AssemblerInstruction{
            opcode: Token::Op { code: Opcode::LOAD },
            operand1: Some(Token::Register { reg: 0 }),
            operand2: Some(Token::IntergerOperand { val: 100 }),
            operand3: None
        }
    )))

    }
}
