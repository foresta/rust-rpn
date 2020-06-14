use crate::ast::Ast;
use crate::tokenizer::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    MissingOperand,
    RemainingOperand,
}

impl ParseError {
    pub fn message(&self) -> String {
        return match self {
            ParseError::MissingOperand => {
                "Invalid RPN Syntax: missing operand. operator should be required two operands."
                    .to_string()
            }
            ParseError::RemainingOperand => {
                "Invalid RPN Syntax: remaining operand. rpn result is one value".to_string()
            }
        };
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    let mut stack: Vec<Ast> = Vec::new();
    for token in tokens {
        match token {
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err(ParseError::MissingOperand);
                }
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                let ast = Ast::Op {
                    op: op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
                stack.push(ast);
            }
            Token::Operand(n) => stack.push(Ast::Num(n)),
        }
    }

    if stack.len() != 1 {
        return Err(ParseError::RemainingOperand);
    }
    return Ok(stack.pop().unwrap());
}

#[cfg(test)]
mod tests {
    use crate::ast::Ast;
    use crate::parser::{parse, ParseError};
    use crate::tokenizer::Token;

    #[test]
    fn test_parse() {
        use crate::operator::Operator;

        assert_eq!(
            parse(vec![Token::Operand(1.0)]).ok().unwrap(),
            Ast::Num(1.0)
        );
        assert_eq!(
            parse(vec![
                Token::Operand(1.0),
                Token::Operand(2.0),
                Token::Operator(Operator::Add)
            ])
            .ok()
            .unwrap(),
            Ast::Op {
                op: Operator::Add,
                lhs: Box::new(Ast::Num(1.0)),
                rhs: Box::new(Ast::Num(2.0))
            }
        );
        assert_eq!(
            parse(vec![Token::Operand(1.0), Token::Operand(2.0)])
                .err()
                .unwrap(),
            ParseError::RemainingOperand
        );
        assert_eq!(
            parse(vec![Token::Operand(1.0), Token::Operator(Operator::Add)])
                .err()
                .unwrap(),
            ParseError::MissingOperand
        );
        assert_eq!(
            parse(vec![
                Token::Operand(1.0),
                Token::Operand(2.0),
                Token::Operator(Operator::Add),
                Token::Operand(3.0),
                Token::Operator(Operator::Mul)
            ])
            .ok()
            .unwrap(),
            Ast::Op {
                op: Operator::Mul,
                lhs: Box::new(Ast::Op {
                    op: Operator::Add,
                    lhs: Box::new(Ast::Num(1.0)),
                    rhs: Box::new(Ast::Num(2.0))
                }),
                rhs: Box::new(Ast::Num(3.0))
            }
        );

        assert_eq!(
            parse(vec![Token::Operand(1.0), Token::Operator(Operator::Add)]),
            Err(ParseError::MissingOperand)
        );
        assert_eq!(
            parse(vec![Token::Operand(1.0), Token::Operand(2.0)]),
            Err(ParseError::RemainingOperand)
        );
    }
}
