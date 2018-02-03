//! This module does all the calculations.
//!
//! For Kila-only calculation operations, please consult submodule kilac.
//! For Kipa-compatible calculation operations, please consult module kipac.

use super::kipac;
pub mod kilac;
pub mod lexer;
use self::lexer::{Token, Lexer};
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Fun {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    Pow,
    Aikavali,
    Abs,
    Log,
    Ln,
    Floor,
    Ceil,
    Sqrt,
    Exp,
    Interpoloi,
    Min,
    Max,
    Sum,
    Med,
    Kesk,
    If,
    SS,
    List,
    Pair,
    Empty,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Ast {
    Node(Vec<Box<Ast>>, Fun),
    Leaf(f64),
    Get(String),
    Empty,
}

fn parse_internal(mut input: Vec<Token>) -> Ast {
    let mut cur = Fun::List;
    let mut nodes: Vec<Box<Ast>> = Vec::new();
    while let Some(token) = input.pop() {
        match token {
            Token::Add => {
                cur = Fun::Add;
            }
            Token::Sub => unimplemented!(),
            Token::Sum => unimplemented!(),
            Token::Div => unimplemented!(),
            Token::Mul => unimplemented!(),
            Token::Mod => unimplemented!(),
            Token::Pow => unimplemented!(),
            Token::Aikavali => unimplemented!(),
            Token::Abs => cur = Fun::Abs,
            Token::Log => cur = Fun::Log,
            Token::Ln => cur = Fun::Ln,
            Token::Floor => cur = Fun::Floor,
            Token::Ceil => cur = Fun::Ceil,
            Token::Sqrt => cur = Fun::Sqrt,
            Token::Exp => cur = Fun::Exp,
            Token::Interpoloi => cur = Fun::Interpoloi,
            Token::Min => cur = Fun::Min,
            Token::Max => cur = Fun::Max,
            Token::Med => cur = Fun::Med,
            Token::Kesk => cur = Fun::Kesk,
            Token::If => cur = Fun::If,
            Token::SS => unimplemented!(),
            Token::ParL => nodes.push(Box::new(parse_internal(input.clone()))),
            Token::ParR => return Ast::Node(nodes, cur),
            Token::BrackL => {
                cur = Fun::List;
                nodes.push(Box::new(parse_internal(input.clone())));
            }
            Token::BrackR => return Ast::Node(nodes, cur),
            Token::Comma => (),
            Token::Num(num) => nodes.push(Box::new(Ast::Leaf(num))),
            Token::Expr(expr) => nodes.push(Box::new(Ast::Get(expr))),
            _ => panic!("Shiet"),
        }
    }
    return Ast::Node(nodes, cur);

}

pub fn parse(mut input: Vec<Token>) -> Ast {
    input.reverse();
    parse_internal(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let tokens = vec![
            Token::Min,
            Token::ParL,
            Token::Num(0.0),
            Token::Num(5.0),
            Token::ParR,
        ];
        let ast = Ast::Node(
            vec![Box::new(Ast::Leaf(0.0)), Box::new(Ast::Leaf(5.0))],
            Fun::Min,
        );
        assert_eq!(ast, parse(tokens));
    }

    #[test]
    fn test_parse_nest() {
        let tokens = vec![
            Token::Min,
            Token::ParL,
            Token::Num(0.0),
            Token::Min,
            Token::ParL,
            Token::Num(5.0),
            Token::Num(6.0),
            Token::ParR,
            Token::ParR,
        ];
        let res = Ast::Node(vec![Box::new(Ast::Leaf(0.0))], Fun::Min);
        assert_eq!(res, parse(tokens));
    }

    #[test]
    fn test_lexparse() {
        let mut lexer = Lexer::new("min(0.0, 5.0, 6.0)");
        let lexed = lexer.lex();
        assert_eq!(Ast::Empty, parse(lexed));
    }
    #[test]
    fn test_sqrt() {
        let mut lexer = Lexer::new("sqrt(5.5)");
        let lexed = lexer.lex();
        assert_eq!(Ast::Empty, parse(lexed));
    }
}
