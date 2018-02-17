//! Module which consists all predefined applicators.
use super::super::ctx::*;
use super::super::{eval, Value};
use super::*;

fn has_get(mut n: Vec<Ast>) -> bool {
    while let Some(t) = n.pop() {
        match t {
            Ast::Get(_) => return true,
            _ => (),
        }
    }
    false
}

fn is_get(n: Ast) -> bool {
    match n {
        Ast::Get(_) => return true,
        _ => false,
    }
}

pub fn optimize(nodes: Vec<Ast>, fun: Fun) -> Ast {
    return match eval(Ast::Node(nodes.clone(), fun)) {
        Ok(Value::Num(n)) => Ast::Leaf(n),
        _ => Ast::Node(nodes, fun)
    };
}

pub fn fix_mulget(mut nodes: Vec<Ast>, fun: Fun) -> Option<Ast> {
    return if fun == Fun::Mul && has_get(nodes.clone()) {
        let a = nodes.pop()?;
        let b = nodes.pop()?;
        if a == Ast::Get("muk".into()) && is_get(b.clone()) ||
            b == Ast::Get("muk".into()) && is_get(a.clone())
        {
            Some(Ast::Get(format!(
                "muk{}",
                match b {
                    Ast::Get(n) => n,
                    _ => panic!("Error"),
                }
            )))
        } else {
            None
        }
    } else {
        None
    };
}

pub fn empty<C: KilaCtx>(nodes: Vec<Ast>, fun: Fun, _: C) -> Ast {
    Ast::Node(nodes, fun)
}

pub fn basic<C: KilaCtx>(nodes: Vec<Ast>, fun: Fun, _: C) -> Ast {
    let t = fix_mulget(nodes.clone(), fun);
    return match t {
        Some(n) => n,
        None => optimize(nodes, fun),
    };
}
