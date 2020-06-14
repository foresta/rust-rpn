mod ast;
mod operator;
mod parser;
mod tokenizer;

/// Calculates single-digit Reverse Polish Notation
///
/// # Examples
/// ```
/// extern crate single_digit_rpn;
/// use single_digit_rpn::rpn;
///
/// let result = rpn("123++").unwrap();
/// assert_eq!(result, 6.0);
/// ```
/// # Errors
///
/// This funtion raises an error
///
/// - if it includes an invalid character (recognized number, +, -, *, and /)
/// - if there are too many operands
/// - if there are too few operators
///
pub fn rpn(expr: &str) -> Result<f64, String> {
    return match tokenizer::tokenize(expr) {
        Ok(tokens) => match parser::parse(tokens) {
            Ok(ast) => Ok(ast.evaluate()),
            Err(err) => Err(err.message()),
        },
        Err(err) => Err(err.message()),
    };
}

#[cfg(test)]
mod tests {
    use crate::rpn;

    #[test]
    fn test_rpn() {
        assert_eq!(rpn("123 + +"), Ok(6.0));
        assert_eq!(rpn("1234+++"), Ok(10.0));
        assert_eq!(rpn("83+5-9+"), Ok(15.0));
        assert_eq!(rpn("893/*4+"), Ok(28.0));
        assert!(rpn("9999+").is_err());
        assert!(rpn("9++++").is_err());
    }
}
