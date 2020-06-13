#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    fn exec(&self, x: f64, y: f64) -> f64 {
        match self {
            Operator::Add => x + y,
            Operator::Sub => x - y,
            Operator::Mul => x * y,
            Operator::Div => x / y,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Operator(Operator),
    Operand(f64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TokenError {
    InvalidChar(char),
}

impl TokenError {
    fn message(&self) -> String {
        return match self {
            TokenError::InvalidChar(c) => format!("Invalid Char: {}", c),
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Ast {
    Num(f64),
    Op {
        op: Operator,
        lhs: Box<Ast>,
        rhs: Box<Ast>,
    },
}

impl Ast {
    fn evaluate(&self) -> f64 {
        match self {
            Ast::Num(n) => *n,
            Ast::Op { op, lhs, rhs } => op.exec(lhs.evaluate(), rhs.evaluate()),
        }
    }
}

fn tokenize(expr: &str) -> Result<Vec<Token>, TokenError> {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ParseError {
    MissingOperand,
    RemainingOperand,
}

impl ParseError {
    fn message(&self) -> String {
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

fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
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

pub fn rpn(expr: &str) -> Result<f64, String> {
    return match tokenize(expr) {
        Ok(tokens) => match parse(tokens) {
            Ok(ast) => Ok(ast.evaluate()),
            Err(err) => Err(err.message()),
        },
        Err(err) => Err(err.message()),
    };
}

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

#[test]
fn test_parse() {
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

#[test]
fn test_evaluate() {
    assert_eq!(Ast::Num(4.0).evaluate(), 4.0);
    assert_eq!(
        Ast::Op {
            op: Operator::Add,
            lhs: Box::new(Ast::Num(1.0)),
            rhs: Box::new(Ast::Num(2.0)),
        }
        .evaluate(),
        3.0
    );
}

#[test]
fn test_rpn() {
    assert_eq!(rpn("123 + +"), Ok(6.0));
    assert_eq!(rpn("1234+++"), Ok(10.0));
    assert_eq!(rpn("83+5-9+"), Ok(15.0));
    assert_eq!(rpn("893/*4+"), Ok(28.0));
    assert!(rpn("9999+").is_err());
    assert!(rpn("9++++").is_err());
}
