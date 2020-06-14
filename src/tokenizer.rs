use crate::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(Operator),
    Operand(f64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenError {
    InvalidChar(char),
}

impl TokenError {
    pub fn message(&self) -> String {
        return match self {
            TokenError::InvalidChar(c) => format!("Invalid Char: {}", c),
        };
    }
}

pub fn tokenize(expr: &str) -> Result<Vec<Token>, TokenError> {
    expr.chars()
        .filter(|c| !c.is_whitespace())
        .map(|e| match e {
            '+' => Ok(Token::Operator(Operator::Add)),
            '-' => Ok(Token::Operator(Operator::Sub)),
            '*' => Ok(Token::Operator(Operator::Mul)),
            '/' => Ok(Token::Operator(Operator::Div)),
            n => match n.to_string().parse::<f64>() {
                Ok(val) => Ok(Token::Operand(val)),
                Err(_) => Err(TokenError::InvalidChar(n)),
            },
        })
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::operator::Operator;
    use crate::tokenizer::{tokenize, Token, TokenError};

    #[test]
    fn test_tokenize() {
        // Check tokenize operand
        assert_eq!(tokenize("1"), Ok(vec![Token::Operand(1.0)]));
        assert_eq!(tokenize("2"), Ok(vec![Token::Operand(2.0)]));
        assert_eq!(tokenize("3"), Ok(vec![Token::Operand(3.0)]));
        assert_eq!(tokenize("4"), Ok(vec![Token::Operand(4.0)]));
        assert_eq!(tokenize("5"), Ok(vec![Token::Operand(5.0)]));
        assert_eq!(tokenize("6"), Ok(vec![Token::Operand(6.0)]));
        assert_eq!(tokenize("7"), Ok(vec![Token::Operand(7.0)]));
        assert_eq!(tokenize("8"), Ok(vec![Token::Operand(8.0)]));
        assert_eq!(tokenize("9"), Ok(vec![Token::Operand(9.0)]));
        assert_eq!(tokenize("0"), Ok(vec![Token::Operand(0.0)]));

        // Check tokenize operand
        assert_eq!(tokenize("+"), Ok(vec![Token::Operator(Operator::Add)]));
        assert_eq!(tokenize("-"), Ok(vec![Token::Operator(Operator::Sub)]));
        assert_eq!(tokenize("*"), Ok(vec![Token::Operator(Operator::Mul)]));
        assert_eq!(tokenize("/"), Ok(vec![Token::Operator(Operator::Div)]));

        // Check tokenize invalid char
        assert_eq!(tokenize("a"), Err(TokenError::InvalidChar('a')));

        // Check whitespace
        assert_eq!(tokenize(" "), Ok(vec![]));
        assert_eq!(tokenize(" 1"), Ok(vec![Token::Operand(1.0)]));
        assert_eq!(tokenize("1 "), Ok(vec![Token::Operand(1.0)]));
        assert_eq!(
            tokenize("1 1"),
            Ok(vec![Token::Operand(1.0), Token::Operand(1.0)])
        );

        // Check tokenize tokens
        assert_eq!(
            tokenize("123+- 45 *"),
            Ok(vec![
                Token::Operand(1.0),
                Token::Operand(2.0),
                Token::Operand(3.0),
                Token::Operator(Operator::Add),
                Token::Operator(Operator::Sub),
                Token::Operand(4.0),
                Token::Operand(5.0),
                Token::Operator(Operator::Mul)
            ])
        );
    }
}
