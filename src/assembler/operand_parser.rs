use super::label_parsers::label_usage;
use super::register_parser::register;
use crate::assembler::Token;
use nom::digit;
use nom::types::CompleteStr;

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
named!(
    pub irstring<CompleteStr,Token>,
    do_parse!(
        tag!("'") >>
        content: take_until!("'") >>
        tag!("'") >>
        (
            Token::IrString { name: content.to_string() }
        )
    )
);

named!(
    pub operand<CompleteStr,Token>,
    alt!(
        interger_operand|
        label_usage|
        register|
        irstring
    )
);

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_parse_string_operand() {
        let result = operand(CompleteStr("'This is Just me testing things lol'"));
        assert!(result.is_ok());
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                Token::IrString {
                    name: "This is Just me testing things lol".to_string()
                }
            ))
        );
    }
}
