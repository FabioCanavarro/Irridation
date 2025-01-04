use crate::instruction::Opcode;
use nom::types::CompleteStr;
use crate::assembler::Token;
use nom::alpha1;

named!(
    pub opcode_load<CompleteStr,Token>,
    do_parse!(
        tag!("load") >> (Token::Op{code: Opcode::LOAD})
    )
);

named!(pub opcode<CompleteStr, Token>,
  do_parse!(
      opcode: alpha1 >>
      (
        {
            Token::Op{code: Opcode::from(opcode)}
        }
      )
  )
);

#[cfg(test)]
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
    
    #[test]
    fn test_parse_opcode() {
        let result = opcode(CompleteStr("load"));
        assert!(result.is_ok());
        let (_,token) = result.unwrap();
        assert_eq!(token,Token::Op { code: Opcode::LOAD });
        let result = opcode(CompleteStr("aoad"));
        assert!(result.is_ok());
        let (_,token) = result.unwrap();
        assert_eq!(token,Token::Op { code: Opcode::IGL });
        let result = opcode(CompleteStr("add"));
        assert!(result.is_ok());
        let (_,token) = result.unwrap();
        assert_eq!(token,Token::Op { code: Opcode::ADD });
        

    }

}
