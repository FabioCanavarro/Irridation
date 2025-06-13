use crate::assembler::Token;
use nom::digit;
use nom::types::CompleteStr;

named!(
    pub register <CompleteStr,Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            reg: digit >>
            (
                Token::Register{
                    reg: reg.parse::<u8>().unwrap()
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_registers() {
        let result = register(CompleteStr("$0"));
        assert!(result.is_ok());
        let result = register(CompleteStr("0"));
        assert!(result.is_err());
        let result = register(CompleteStr("$u"));
        assert!(result.is_err());
    }
}
