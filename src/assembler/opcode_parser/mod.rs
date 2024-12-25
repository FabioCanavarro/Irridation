use crate::instruction::Opcode;
use nom::types::CompleteStr;
use crate::assembler::Token;
named!(
    opcode_load<CompleteStr,Token>,
    do_parse!(
        tag!("load") >> (Token::Op{code: Opcode::LOAD})
        // looks for load, and if founded returns the enum
    )
);

mod tests{
    use super::*;

    #[test]
    fn test_parse_opcode_load() {
        let result = opcode_load(CompleteStr("load"));
        assert!(result.is_ok());
        let (rest,token) = result.unwrap();
        assert_eq!(token,Token::Op { code: Opcode::LOAD });
        assert_eq!(rest,CompleteStr(""));

        let result = opcode_load(CompleteStr("aopload"));
        assert!(result.is_err());
    }


}
