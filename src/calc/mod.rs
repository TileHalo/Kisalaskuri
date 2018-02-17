//! This module does all the calculations.
//! For Kila-only calculation operations, please consult submodule kilac.
//! For Kipa-compatible calculation operations, please consult module kipac.

pub mod kilac;
pub mod lexer;
pub mod parser;

use self::lexer::lex;
use self::parser::{Fun, Ast, parse};
use super::kipac;

macro_rules! cond {
    ($e:expr) => (
        Ok(if $e {
            1.0
        } else {
            0.0
        });
    )
}

pub fn calculate_err(s: String) -> Result<f64, String> {
    let parsed = try!(parse(lex(&s)));
    eval(parsed)

}

pub fn calculate(s: String) -> f64 {
    let parsed = parse(lex(&s)).ok().unwrap();
    eval(parsed).ok().unwrap()

}

pub fn eval(ast: Ast) -> Result<f64, String> {
    return match ast {
        Ast::Empty => panic!("Met empty abstract syntax tree node {:?}", ast),
        Ast::Leaf(num) => Ok(num),
        Ast::Node(vec, fun) => {
            let mut res: Vec<f64> = Vec::new();
            for i in vec {
                let evald = try!(eval(i.clone()));
                res.push(evald);
            }
            match fun {
                Fun::Abs => Ok(kipac::abs(res[0])),
                Fun::Log => Ok(kipac::log(res[0])),
                Fun::Aikavali => Ok(kipac::aikavali(res[0], res[1])),
                Fun::Ln => Ok(kipac::ln(res[0])),
                Fun::Floor => Ok(kipac::floor(res[0])),
                Fun::Ceil => Ok(kipac::ceil(res[0])),
                Fun::Sqrt => Ok(kipac::sqrt(res[0])),
                Fun::Exp => Ok(kipac::exp(res[0])),
                Fun::Pow => Ok(kipac::pow(res[0], res[1])),
                Fun::Interpoloi => Ok(kipac::interpoloi(res[0], res[1], res[2], res[3], 0.0)),
                Fun::Aikainterp => Ok(kipac::interpoloi(res[0], res[1], res[1]+res[2], res[2], 0.0)),
                Fun::Min => Ok(kipac::min(res)),
                Fun::Max => Ok(kipac::max(res)),
                Fun::Sum | Fun::Add => Ok(kipac::sum(res)),
                Fun::Med => Ok(kipac::median(res)),
                Fun::Kesk => Ok(kipac::mean(res)),
                Fun::Logb => Ok(kipac::ln(res[1])/kipac::ln(res[0])),
                Fun::Div => Ok(res[0] / res[1]),
                Fun::Mul => Ok(res[0] * res[1]),
                Fun::Sub => Ok(res[0] - res[1]),
                Fun::Mod => Ok(res[0] % res[1]),
                Fun::Minus => Ok(-1.0 * res[0]),
                Fun::Plus => Ok(res[0]),
                Fun::Eq => cond!(res[0] == res[1]),
                Fun::Neq => cond!(res[0] != res[1]),
                Fun::Ge => cond!(res[0] <= res[1]),
                Fun::Gt => cond!(res[0] <  res[1]),
                Fun::Le => cond!(res[0] >= res[1]),
                Fun::Lt => cond!(res[0] > res[1]),
                Fun::If => Ok(res.clone()[res[0] as usize+1]),
                Fun::Sin => Ok(f64::sin(res[0])),
                Fun::Cos => Ok(f64::cos(res[0])),
                Fun::Tan => Ok(f64::tan(res[0])),
                Fun::Arcsin => Ok(f64::asin(res[0])),
                Fun::Arccos => Ok(f64::acos(res[0])),
                Fun::Arctan => Ok(f64::atan(res[0])),
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
