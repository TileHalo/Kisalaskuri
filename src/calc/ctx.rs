//! Context module. This module provides types that can deal with
//! context and other things.

use super::parser::Ast;

#[derive(Debug, Clone)]
pub struct EmptyCtx;

/// Classes that implement KilaCtx returns AST node,
/// that corresponds to the type of getter.
/// Thus either List or Leaf is returned.
/// Can also return empty, which signals for empty getter.
pub trait KilaCtx {
    fn get(String) -> Result<Ast, ()>;
}

impl KilaCtx for EmptyCtx {
    fn get(_: String) -> Result<Ast, ()> {
        Ok(Ast::Empty)
    }
}
