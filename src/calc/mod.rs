//! This module does all the calculations.
//!
//! For Kila-only calculation operations, please consult submodule kilac.
//! For Kipa-compatible calculation operations, please consult module kipac.

pub mod kilac;
pub mod lexer;
pub mod lisp;
pub mod parser;

use self::lisp::lispify;
use self::lexer::Lexer;
use self::parser::{Fun, Ast, parse};
use super::kipac;

pub fn eval(ast: Ast) -> f64 {
    return match ast {
        Ast::Empty => panic!("Met empty abstract syntax tree node {:?}", ast),
        Ast::Leaf(num) => num,
        Ast::Node(vec, fun) => {
            let mut res: Vec<f64> = Vec::new();
            for i in vec {
                res.push(eval(i.as_ref().clone()));
            }
            return match fun {
                Fun::Abs => kipac::abs(res[0]),
                Fun::Log => kipac::log(res[0]),
                Fun::Aikavali => kipac::aikavali(res[0], res[1]),
                Fun::Ln => kipac::ln(res[0]),
                Fun::Floor => kipac::floor(res[0]),
                Fun::Ceil => kipac::ceil(res[0]),
                Fun::Sqrt => kipac::sqrt(res[0]),
                Fun::Exp => kipac::exp(res[0]),
                Fun::Mod => kipac::kmod(res[0], res[1]),
                Fun::Pow => kipac::pow(res[0], res[1]),
                Fun::Interpoloi => kipac::abs(res[0]),
                Fun::Min => kipac::min(res),
                Fun::Max => kipac::max(res),
                Fun::Sum | Fun::Add => kipac::sum(res),
                Fun::Med => kipac::median(res),
                Fun::Kesk => kipac::mean(res),
                Fun::Div => res[0]/res[0],
                Fun::Mul => res[0]*res[0],
                _ => unimplemented!("Function {:?}", fun),
            };
        }
        _ => unimplemented!("{:?}", ast),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut lexer = Lexer::new("5+6");
        assert_eq!(11.0, eval(parse(lispify(lexer.lex()))));
    }
    #[test]
    fn test_min() {
        let mut lexer = Lexer::new("min(5, 6)");
        assert_eq!(5.0, eval(parse(lexer.lex())));
    }
    #[test]
    fn test_arithmetic() {
        let mut lexer = Lexer::new("5+5-((6*12)/2)");
        assert_eq!(-26.0, eval(parse(lispify(lexer.lex()))));
    }
    #[test]
    fn test_min_plus() {
        let mut lexer = Lexer::new("min(5, 6, 0+4, 10, 5, 1)");
        let lexed = lexer.lex();
        println!("{:?}", lexed);
        let lexd = lispify(lexed);
        println!("{:?}", lexd);
        println!("{:?}", parse(lexd.clone()));
        assert_eq!(1.0, eval(parse(lexd)));
    }
}
