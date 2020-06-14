#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    /// Arithmetically executes the operator
    pub fn exec(&self, x: f64, y: f64) -> f64 {
        match self {
            Operator::Add => x + y,
            Operator::Sub => x - y,
            Operator::Mul => x * y,
            Operator::Div => x / y,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::operator::Operator;

    #[test]
    fn test_exec() {
        assert_eq!(Operator::Add.exec(1.0, 2.0), 3.0);
        assert_eq!(Operator::Sub.exec(5.0, 2.0), 3.0);
        assert_eq!(Operator::Sub.exec(1.0, 2.0), -1.0);
        assert_eq!(Operator::Mul.exec(4.0, 2.0), 8.0);
        assert_eq!(Operator::Div.exec(8.0, 2.0), 4.0);
    }
}
