use nom::{alphanumeric,multispace, types::CompleteStr};
use super::Token;

// Looks for a user-defined label such as label1:
named!(
    pub label_declaration<CompleteStr,Token>,
    ws!(
        do_parse!(
        name: alphanumeric >>
        tag!(":") >>
        opt!(multispace) >>
            (
                Token::LabelDeclaration { name: name.to_string() }
            )
        )
    )
);


// Look for its usage such as @label1
named!(
    pub label_usage<CompleteStr,Token>,
    ws!(
        do_parse!(
        tag!("@") >>
        name: alphanumeric >>
        opt!(multispace) >>
            (
                Token::LabelUsage { name: name.to_string() }
            )
        )
    )
);

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_label_declaration() {
        let result = label_declaration(CompleteStr("test:"));
        assert!(result.is_ok());
        let (_,token) = result.unwrap();
        assert_eq!(token,Token::LabelDeclaration { name: "test".to_string()});
        let result = label_declaration(CompleteStr("test"));
        assert!(result.is_err());
    }

    #[test]
    fn test_label_usage() {
        let result = label_usage(CompleteStr("@test"));
        assert!(result.is_ok());
        let (_,token) = result.unwrap();
        assert_eq!(token,Token::LabelUsage { name: "test".to_string() });
        let result = label_usage(CompleteStr("test"));
        assert!(result.is_err());
    }
}
