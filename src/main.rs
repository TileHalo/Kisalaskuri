#![allow(unused_imports,dead_code)]
extern crate kilac;
use std::io;

use kilac::*;

fn main() {
    let mut eva = String::new();
    while let Ok(_) = io::stdin().read_line(&mut eva) {
        let lexed = calc::lexer::lex(&eva.clone().trim());
        let ast = calc::parser::parse(lexed).ok().unwrap();
        match calc::eval(ast).ok().unwrap() {
            kilac::calc::Value::Num(n) => println!("{}", n),
            kilac::calc::Value::Vec(n) => println!("{:?}", n)
        }
        eva = String::new();
    }
}
