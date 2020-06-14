use crate::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Num(f64),
    Op {
        op: Operator,
        lhs: Box<Ast>,
        rhs: Box<Ast>,
    },
}

impl Ast {
    pub fn evaluate(&self) -> f64 {
        match self {
            Ast::Num(n) => *n,
            Ast::Op { op, lhs, rhs } => op.exec(lhs.evaluate(), rhs.evaluate()),
        }
    }
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
