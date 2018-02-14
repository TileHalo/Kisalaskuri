//! This module does all the calculations.
//!
//! For Kila-only calculation operations, please consult submodule kilac.
//! For Kipa-compatible calculation operations, please consult module kipac.

pub mod kilac;
pub mod lexer;
pub mod lisp;
pub mod parser;

use self::lexer::lex;
use self::parser::{Fun, Ast, parse};
use super::kipac;

macro_rules! cond {
    ($e:expr) => (
        if $e {
            1.0
        } else {
            0.0
        };
    )
}

pub fn calculate(s: String) -> f64 {
    eval(parse(lisp::lispify(lex(&s))))

}

pub fn eval(ast: Ast) -> f64 {
    return match ast {
        Ast::Empty => panic!("Met empty abstract syntax tree node {:?}", ast),
        Ast::Leaf(num) => num,
        Ast::Node(vec, fun) => {
            let mut res: Vec<f64> = Vec::new();
            for i in vec {
                res.push(eval(i.clone()));
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
                Fun::Pow => kipac::pow(res[1], res[0]),
                Fun::Interpoloi => kipac::interpoloi(res[0], res[1], res[2], res[3], 0.0),
                Fun::Aikainterp => kipac::interpoloi(res[0], res[1], res[1]+res[2], res[2], 0.0),
                Fun::Min => kipac::min(res),
                Fun::Max => kipac::max(res),
                Fun::Sum | Fun::Add => kipac::sum(res),
                Fun::Med => kipac::median(res),
                Fun::Kesk => kipac::mean(res),
                Fun::Logb => kipac::ln(res[0])/kipac::ln(res[1]),
                Fun::Div => res[1] / res[0],
                Fun::Mul => res[0] * res[1],
                Fun::Sub => res[1] - res[0],
                Fun::Mod => res[1] % res[0],
                Fun::Minus => -1.0*res[0],
                Fun::Plus => res[0],
                Fun::Eq => cond!(res[1] == res[0]),
                Fun::Neq => cond!(res[1] != res[0]),
                Fun::Ge => cond!(res[1] <= res[0]),
                Fun::Gt => cond!(res[1] <  res[0]),
                Fun::Le => cond!(res[1] >= res[0]),
                Fun::Lt => cond!(res[1] > res[0]),
                Fun::If => res.clone()[res[0] as usize+1],
                Fun::Sin => f64::sin(res[0]),
                Fun::Cos => f64::cos(res[0]),
                Fun::Tan => f64::tan(res[0]),
                Fun::Arcsin => f64::asin(res[0]),
                Fun::Arccos => f64::acos(res[0]),
                Fun::Arctan => f64::atan(res[0]),
                _ => unimplemented!("Function {:#?}", fun),
            };
        },
        Ast::Get(_) => panic!("Eval cannot handle Get: {:#?}", ast),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let s = "5+6";
        assert_eq!(11.0, calculate(s.into()));
    }
    #[test]
    fn test_sub() {
        let s = "12-2";
        assert_eq!(10.0, calculate(s.into()));
    }
    #[test]
    fn test_div() {
        let s = "12/2";
        assert_eq!(6.0, calculate(s.into()));
    }
    #[test]
    fn test_mul() {
        let s = "12*2";
        assert_eq!(24.0, calculate(s.into()));
    }
    #[test]
    fn test_mod() {
        let s = "12%2";
        assert_eq!(0.0, calculate(s.into()));
    }
    #[test]
    fn test_pow() {
        let s = "2^2";
        assert_eq!(4.0, calculate(s.into()));
    }
    #[test]
    fn test_arithmetic() {
        let s = "5+5-6*12/2";
        assert_eq!(-26.0, calculate(s.into()));
    }
    #[test]
    fn test_min() {
        assert_eq!(2.0, calculate("min(5, 10, 2)".into()));
        assert_eq!(-10.0, calculate("min(5, -10, 2)".into()));
    }
    #[test]
    fn unary() {
        assert_eq!(-10.0, calculate("-5*2".into()));
    }
}
