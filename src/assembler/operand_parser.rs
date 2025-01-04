use nom::types::CompleteStr;
use crate::assembler::Token;
use nom::digit;


named!(
    pub interger_operand<CompleteStr,Token>,
    ws!(
        do_parse!(
            tag!("#") >> val: digit >>
                (
                    Token::IntergerOperand{
                            val: val.parse::<i32>().unwrap()
                    }
            )
        )
    )
);

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_parse_interger_operand() {
        let result = interger_operand(CompleteStr("#0"));
        assert!(result.is_ok());
        let result = interger_operand(CompleteStr("0"));
        assert!(result.is_err());
        let result = interger_operand(CompleteStr("#u"));
        assert!(result.is_err());
    }

}
