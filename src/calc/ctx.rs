//! Context module. This module provides types that can deal with
//! context and other things.

use super::parser::Ast;
/// Empty object so that bunch of simpler internals can be implemented
#[derive(Debug, Clone)]
pub struct EmptyCtx;

/// Classes that implement KilaCtx returns AST node,
/// that corresponds to the type of getter.
/// Thus either List or Leaf is returned.
/// Can also return empty, which signals for empty getter.
pub trait KilaCtx: Clone {
    fn get(&self, String) -> Result<Ast, String>;
}

impl KilaCtx for EmptyCtx {
    fn get(&self, _: String) -> Result<Ast, String> {
        Ok(Ast::Empty)
    }
}
