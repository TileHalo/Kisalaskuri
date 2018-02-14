//! This module provides Kipa-compatible calculations.
//! Kipa currently supports following functions:
//!
//! ```text
//! aikavali(a, b)
//! abs(x)
//! log(x)
//! ln(x)
//! floor(x)
//! ceil(x)
//! sqrt(x)
//! exp(x)
//! mod(x, y)
//! pow(x,y)
//! interpoloi(x,x1,y1,x2,(y2=0))
//! min(X)
//! max(X)
//! sum(X)
//! med(X)
//! kesk(X)
//! if(cond, true, false)
//! ```
//!
//! Operators supported are
//!
//! ```text
//! +
//! -
//! *
//! /
//! <
//! >
//! ==
//! !=
//! <=
//! >=
//! ```
//!
//! And finally these examples are known to work
//!
//! ```text
//! a+b+c (Sum)
//! 1/a (inverse number)
//! a (teams input)
//! .a (all series inputs)
//! .a.vartio (input of a team)
//! min(.a) (minimum of all series inputs)
//! muk and ..mukana (all inputs of teams that belong to competition)
//! max(.a*muk) (maximum of all teams that are in the competition)
//! vartio (teams number)
//! .b.a.vartio (input a for team from subtask b)
//! ..start.c.a.vartio (input a for team from subtask c from task start)
//! ```
//! In addition to these Kilac supports other operators too.

/// Function aikavali calculates the time difference between two inputs
/// in seconds across two dates.
pub fn aikavali(a: f64, b: f64) -> f64 {
    let s = b - a;
    if s < 0.0 {
        return s + 86400.0;
    }
    s
}
/// Returns absolute value of a number.
pub fn abs(x: f64) -> f64 {
    x.abs()
}
/// Takes log(x, 10)
pub fn log(x: f64) -> f64 {
    x.log10()
}
/// Takes natural logarithm of a number.
pub fn ln(x: f64) -> f64 {
    x.ln()
}
/// Takes floor of a number
pub fn floor(x: f64) -> f64 {
    x.floor()
}
/// Takes ceil of a number
pub fn ceil(x: f64) -> f64 {
    x.ceil()
}
/// Takes square root of number
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}
/// exp(x) or e^x
pub fn exp(x: f64) -> f64 {
    x.exp()
}
/// Modulo of numbers a and b
pub fn kmod(a: f64, b: f64) -> f64 {
    a % b
}
/// Raises number a to power of b
pub fn pow(a: f64, b: f64) -> f64 {
    a.powf(b)
}
#[allow(unused_variables)]
/// Raw interpolation.
pub fn interpoloi(x: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    y1 * (x - x2) / (x1 - x2)
}
/// Returns minimum value of f64 vector.
pub fn min(x: Vec<f64>) -> f64 {
    x.iter().cloned().fold(0. / 0., f64::min)
}
/// Returns maximum value of f64 vector.
pub fn max(x: Vec<f64>) -> f64 {
    x.iter().cloned().fold(0. / 0., f64::max)
}
/// Takes sum of all values in a vector.
pub fn sum(x: Vec<f64>) -> f64 {
    x.iter().sum()
}
/// Takes mean of vector
pub fn mean(x: Vec<f64>) -> f64 {
    let a = x.len();
    let b: f64 = x.iter().sum();
    b / (a as f64)
}
/// Calculates median of a vector
pub fn median(x: Vec<f64>) -> f64 {
    let a = sort(x);
    let ln = a.len();
    if ln % 2 == 0 {
        return (a[(ln / 2)] + a[(ln / 2) - 1]) / 2.0;
    } else {
        let b = ((ln as f64) / 2.0).floor() as usize;
        a[b]
    }
}

/// Sorts list of floats
fn sort(x: Vec<f64>) -> Vec<f64> {
    let mut a = x.clone();
    a.sort_by(|a, b| a.partial_cmp(b).unwrap());
    a
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::E;
    #[test]
    fn test_aikavali() {
        assert_eq!(5.0, aikavali(5.0, 10.0));
        assert_eq!(0.0, aikavali(10.0, 10.0));
        assert_eq!(100.0, aikavali(86400.0, 100.0));
    }
    #[test]
    fn test_abs() {
        assert_eq!(5.0, abs(-5.0));
        assert_eq!(5.0, abs(5.0));
        assert_eq!(25.5555, abs(25.5555));
        assert_eq!(25.50, abs(-25.50));
    }
    #[test]
    fn test_log() {
        assert_eq!(0.0, log(1.0));
        assert_eq!(1.0, log(10.0));
        assert_eq!(0.6989700043360189, log(5.0));
    }
    #[test]
    fn test_ln() {
        assert_eq!(0.0, ln(1.0));
        assert_eq!(1.0, ln(E));
        assert_eq!(1.6094379124341003, ln(5.0));
    }
    #[test]
    fn test_floor() {
        assert_eq!(1.0, floor(1.0));
        assert_eq!(1.0, floor(1.6));
        assert_eq!(1.0, floor(1.999));
        assert_eq!(1.0, floor(1.5));
        assert_eq!(-1.0, floor(-0.4));
    }
    #[test]
    fn test_ceil() {
        assert_eq!(1.0, ceil(1.0));
        assert_eq!(2.0, ceil(1.6));
        assert_eq!(2.0, ceil(1.999));
        assert_eq!(2.0, ceil(1.5));
        assert_eq!(-1.0, ceil(-1.4));
    }
    #[test]
    fn test_sqrt() {}
    #[test]
    fn test_exp() {
        assert_eq!(E, exp(1.0));
        assert_eq!(1.0, exp(0.0));
    }
    #[test]
    fn test_mod() {
        assert_eq!(0.0, kmod(4.0, 2.0));
        assert_eq!(1.0, kmod(3.0, 2.0));
    }
    #[test]
    fn test_pow() {
        assert_eq!(4.0, pow(2.0, 2.0));
        assert_eq!(8.0, pow(2.0, 3.0));
        assert_eq!(-8.0, pow(-2.0, 3.0));
        assert_eq!(2.0, pow(4.0, 0.5));
        assert_eq!(0.03125, pow(4.0, -2.5));
    }
    #[test]
    fn test_interpoloi() {}
    // These two tests should be done better. Now we can't know what happens.
    #[test]
    fn test_max() {
        assert_eq!(5.0, max(vec![5.0, 1.0, -10.0, 4.99999, 2.5]))
    }
    #[test]
    fn test_min() {
        assert_eq!(-10.0, min(vec![5.0, 1.0, -10.0, 4.99999, 2.5]));
        assert_eq!(0.0, min(vec![0.0, 1.0]));
    }
    #[test]
    fn test_mean() {
        assert_eq!(2.0, mean(vec![1.0, 2.0, 3.0]));
    }
    #[test]
    fn test_median() {
        assert_eq!(2.0, median(vec![1.0, 2.0, 3.0]));
    }
}
