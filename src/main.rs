#![allow(unused_imports,dead_code)]
extern crate kilac;
use std::io;

use kilac::*;

fn main() {
    let mut eva = String::new();
    while let Ok(_) = io::stdin().read_line(&mut eva) {
        let lexed = calc::lexer::lex(&eva.clone().trim());
        let ast = calc::parser::parse(lexed);
        println!("AST: {:#?}", ast);
        println!("Result: {}", calc::eval(ast));
        eva = String::new();
    }
}
