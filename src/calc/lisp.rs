//! This module handles translating to lisp

use super::lexer::*;
// There has to be some edge case that I am currently missing.
/// This step translates mathematical operators to prefix notation
pub fn lispify(input: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();
    let mut iterator = input.iter().peekable();
    while let Some(token) = iterator.next() {
        match iterator.peek() {
            Some(&t) => {
                match t.clone() {
                    Token::Add | Token::Sub | Token::Div | Token::Mul | Token::Imod |
                    Token::Ipow => {
                        output.push(t.clone());
                        iterator.next();
                    }
                    _ => (),
                }
            }
            None => (), 
        }
        output.push(token.clone());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lispify_fn() {
        let lexer = lex("min(5 + (5 + 5) + 5, 10.0)");
        let res = lex("min(+ 5 (+ 5 5 +) 5, 10.0)");
        assert_eq!(res, lispify(lexer));
    }
}
