//! This module handles translating to lisp

use super::lexer::*;
// There has to be some edge case that I am currently missing.
/// This step translates mathematical operators to prefix notation
pub fn lispify(input: Vec<Token>) -> Vec<Token> {
    let mut infix = false;
    let mut pall = false;
    let mut new: Vec<Token> = Vec::new();
    for item in input.iter() {
        match *item {
            Token::Sub | Token::Add | Token::Div | Token::Mul | Token::Mod | Token::Pow => {
                if !infix {
                    let tmp = new.pop().unwrap();
                    new.push(item.clone());
                    new.push(tmp);
                    infix = true;
                }
            }
            Token::ParL => {
                pall = true;
                infix = false;
                new.push(item.clone());
            }
            Token::ParR => {
                if pall {
                    infix = true;
                    pall = false;
                }
                new.push(item.clone());
            }
            Token::Comma => {
                infix = false;
                new.push(item.clone());
            } 
            _ => new.push(item.clone()),
        }
    }
    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lispify() {
        let mut lexer = Lexer::new("5 + 2");
        assert_eq!(
            vec![Token::Add, Token::Num(5.0), Token::Num(2.0)],
            lispify(lexer.lex())
        );
    }
    #[test]
    fn test_lispify_many() {
        let mut lexer = Lexer::new("5 + 2 + 3 + 4.0");
        assert_eq!(
            vec![
                Token::Add,
                Token::Num(5.0),
                Token::Num(2.0),
                Token::Num(3.0),
                Token::Num(4.0),
            ],
            lispify(lexer.lex())
        );
    }
    #[test]
    fn test_lispify_nest() {
        let mut lexer = Lexer::new("5 + (5 + 5)");
        assert_eq!(
            vec![
                Token::Add,
                Token::Num(5.0),
                Token::ParL,
                Token::Add,
                Token::Num(5.0),
                Token::Num(5.0),
                Token::ParR,
            ],
            lispify(lexer.lex())
        );
    }
    #[test]
    fn test_lispify_nest_continue() {
        let mut lexer = Lexer::new("5 + (5 + 5) + 5");
        assert_eq!(
            vec![
                Token::Add,
                Token::Num(5.0),
                Token::ParL,
                Token::Add,
                Token::Num(5.0),
                Token::Num(5.0),
                Token::ParR,
                Token::Num(5.0),
            ],
            lispify(lexer.lex())
        );
    }
    #[test]
    fn test_lispify_fn() {
        let mut lexer = Lexer::new("min(5 + (5 + 5) + 5, 10.0)");
        let mut res = Lexer::new("min(+ 5 (+ 5 5) 5, 10.0)");
        assert_eq!(res.lex(), lispify(lexer.lex()));
    }
}
