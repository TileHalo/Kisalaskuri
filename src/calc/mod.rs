//! This module does all the calculations.
//! For Kila-only calculation operations, please consult submodule kilac.
//! For Kipa-compatible calculation operations, please consult module kipac.

pub mod kilac;
pub mod lexer;
pub mod parser;
pub mod ctx;

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
macro_rules! value {
    ($i:ident, $e:expr) => (
        Ok(Value::$i($e))
    )
}

#[derive(Debug, Clone)]
pub enum Value {
    Num(f64),
    Vec(Vec<f64>),
}

pub fn calculate_err(s: String) -> Result<f64, String> {
    let parsed = try!(parse(lex(&s)));
    return match eval(parsed) {
        Ok(p) => {
            match p {
                Value::Num(n) => Ok(n),
                Value::Vec(n) => Err(format!("Got Vector instead of number: {:#?}", n)),
            }
        }
        Err(p) => Err(p),
    };
}

pub fn calculate(s: String) -> f64 {
    let parsed = parse(lex(&s)).ok().unwrap();
    return match eval(parsed).ok().unwrap() {
        Value::Num(n) => n,
        Value::Vec(n) => panic!("Got Vector instead of number: {:#?}", n),
    };
}

pub fn eval(ast: Ast) -> Result<Value, String> {
    return match ast {
        Ast::Empty => panic!("Met empty abstract syntax tree node {:?}", ast),
        Ast::Leaf(num) => value!(Num, num),
        Ast::Node(vec, fun) => {
            let mut res: Vec<f64> = Vec::new();
            for i in vec {
                match try!(eval(i.clone())) {
                    Value::Num(n) => res.push(n),
                    Value::Vec(v) => {
                        res = v.clone();
                        break;
                    }
                }
            }
            match fun {
                Fun::Abs => value!(Num, kipac::abs(res[0])),
                Fun::Log => value!(Num, kipac::log(res[0])),
                Fun::Aikavali => value!(Num, kipac::aikavali(res[0], res[1])),
                Fun::Ln => value!(Num, kipac::ln(res[0])),
                Fun::Floor => value!(Num, kipac::floor(res[0])),
                Fun::Ceil => value!(Num, kipac::ceil(res[0])),
                Fun::Sqrt => value!(Num, kipac::sqrt(res[0])),
                Fun::Exp => value!(Num, kipac::exp(res[0])),
                Fun::Pow => value!(Num, kipac::pow(res[0], res[1])),
                Fun::Interpoloi => {
                    value!(Num, kipac::interpoloi(res[0], res[1], res[2], res[3], 0.0))
                }
                Fun::Aikainterp => {
                    value!(
                        Num,
                        kipac::interpoloi(res[0], res[1], res[1] + res[2], res[2], 0.0)
                    )
                }
                Fun::Min => value!(Num, kipac::min(res)),
                Fun::Max => value!(Num, kipac::max(res)),
                Fun::Sum | Fun::Add => value!(Num, kipac::sum(res)),
                Fun::Med => value!(Num, kipac::median(res)),
                Fun::Kesk => value!(Num, kipac::mean(res)),
                Fun::Logb => value!(Num, kipac::ln(res[1]) / kipac::ln(res[0])),
                Fun::Div => value!(Num, res[0] / res[1]),
                Fun::Mul => value!(Num, res[0] * res[1]),
                Fun::Sub => value!(Num, res[0] - res[1]),
                Fun::Mod => value!(Num, res[0] % res[1]),
                Fun::Minus => value!(Num, -1.0 * res[0]),
                Fun::Plus => value!(Num, res[0]),
                Fun::Eq => value!(Num, cond!(res[0] == res[1])),
                Fun::Neq => value!(Num, cond!(res[0] != res[1])),
                Fun::Ge => value!(Num, cond!(res[0] <= res[1])),
                Fun::Gt => value!(Num, cond!(res[0] < res[1])),
                Fun::Le => value!(Num, cond!(res[0] >= res[1])),
                Fun::Lt => value!(Num, cond!(res[0] > res[1])),
                Fun::If => value!(Num, res.clone()[res[0] as usize + 1]),
                Fun::Sin => value!(Num, f64::sin(res[0])),
                Fun::Cos => value!(Num, f64::cos(res[0])),
                Fun::Tan => value!(Num, f64::tan(res[0])),
                Fun::Arcsin => value!(Num, f64::asin(res[0])),
                Fun::Arccos => value!(Num, f64::acos(res[0])),
                Fun::Arctan => value!(Num, f64::atan(res[0])),
                Fun::List => value!(Vec, res),
                _ => Err(format!("Function {:#?}", fun)),
            }
        }
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
