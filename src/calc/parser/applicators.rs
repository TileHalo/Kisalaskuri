//! Module which consists all predefined applicators for parsing function.
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

/// Tries to optimize a node by running it through evaluator. This applicator
/// is used by default.
pub fn optimize<C: KilaCtx>(nodes: Vec<Ast>, fun: Fun, c: C) -> Ast {
    return match eval(Ast::Node(nodes.clone(), fun), c) {
        Ok(Value::Num(n)) => Ast::Leaf(n),
        _ => Ast::Node(nodes, fun),
    };
}

/// This function fixes the kipas weird way of using multiplication operation
/// in ..mux*a getter.
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

/// Empty applicator. Does really nothing
pub fn empty<C: KilaCtx>(nodes: Vec<Ast>, fun: Fun, _: C) -> Ast {
    Ast::Node(nodes, fun)
}

/// The default applicator. This should be the only one to be used.
pub fn basic<C: KilaCtx>(nodes: Vec<Ast>, fun: Fun, c: C) -> Ast {
    let t = fix_mulget(nodes.clone(), fun);
    return match t {
        Some(n) => {
            match n {
                Ast::Get(s) => {
                    match c.get(s.clone()) {
                        Ok(l) => l,
                        _ => Ast::Get(s),
                    }
                }
                _ => n,
            }
        }
        None => {
            optimize(nodes, fun, c)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize() {
        assert_eq!(Ast::Leaf(12.0), optimize(vec![Ast::Leaf(5.0), Ast::Leaf(7.0)], Fun::Add, EmptyCtx));
    }
}
