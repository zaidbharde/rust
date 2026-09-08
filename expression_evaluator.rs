//! Evaluate non-negative arithmetic expressions with +, -, *, /, and parentheses.

#[derive(Debug, PartialEq)]
pub enum EvalError { UnexpectedToken, DivisionByZero, UnclosedParenthesis }

pub fn evaluate(input: &str) -> Result<i64, EvalError> {
    let tokens: Vec<char> = input.chars().filter(|c| !c.is_whitespace()).collect();
    let mut parser = Parser { tokens, position: 0 };
    let value = parser.expression()?;
    if parser.position != parser.tokens.len() { return Err(EvalError::UnexpectedToken); }
    Ok(value)
}

struct Parser { tokens: Vec<char>, position: usize }
impl Parser {
    fn expression(&mut self) -> Result<i64, EvalError> {
        let mut value = self.term()?;
        while let Some(operator) = self.peek() {
            if operator != '+' && operator != '-' { break; }
            self.position += 1;
            let right = self.term()?;
            value = if operator == '+' { value + right } else { value - right };
        }
        Ok(value)
    }
    fn term(&mut self) -> Result<i64, EvalError> {
        let mut value = self.factor()?;
        while let Some(operator) = self.peek() {
            if operator != '*' && operator != '/' { break; }
            self.position += 1;
            let right = self.factor()?;
            if operator == '/' && right == 0 { return Err(EvalError::DivisionByZero); }
            value = if operator == '*' { value * right } else { value / right };
        }
        Ok(value)
    }
    fn factor(&mut self) -> Result<i64, EvalError> {
        if self.peek() == Some('(') {
            self.position += 1;
            let value = self.expression()?;
            if self.peek() != Some(')') { return Err(EvalError::UnclosedParenthesis); }
            self.position += 1;
            return Ok(value);
        }
        let start = self.position;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) { self.position += 1; }
        if start == self.position { return Err(EvalError::UnexpectedToken); }
        self.tokens[start..self.position].iter().collect::<String>().parse()
            .map_err(|_| EvalError::UnexpectedToken)
    }
    fn peek(&self) -> Option<char> { self.tokens.get(self.position).copied() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn respects_precedence() { assert_eq!(evaluate("2 + 3 * (4 - 1)"), Ok(11)); }
    #[test] fn rejects_zero_division() { assert_eq!(evaluate("8 / 0"), Err(EvalError::DivisionByZero)); }
}
