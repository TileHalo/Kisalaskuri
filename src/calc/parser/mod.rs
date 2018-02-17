pub mod applicators;
use super::lexer::Token;

macro_rules! arity {
    ($e:expr, $b:expr) => (
        match $e {
            Fun::Add | Fun::Sub | Fun::Div | Fun::Mul | Fun::Mod | Fun::Pow => 2,
            Fun::Eq | Fun::Neq | Fun::Ge | Fun::Gt | Fun::Le | Fun::Lt => 2,
            Fun::Aikavali | Fun::Logb => 2,
            Fun::Log | Fun::Ln | Fun::Floor | Fun::Ceil | Fun::Sqrt | Fun::Exp => 1,
            Fun::Sin | Fun::Cos | Fun::Tan | Fun::Arcsin | Fun::Arccos | Fun::Arctan => 1,
            Fun::If | Fun::Aikainterp => 3,
            Fun::Minus | Fun::Plus => 1,
            _ => $b.pop().unwrap()
        };
    )
}

macro_rules! children {
    ($e:expr, $b:expr) => (
        {
            let mut v: Vec<Ast> = Vec::new();
            for _ in 0..$e {
                v.push($b.pop().unwrap())
            }
            v.iter().rev().cloned().collect::<Vec<Ast>>()
        };
    )
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Fun {
    Add,
    Sub,
    Div,
    Mul,
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
    Logb,
    Interp,
    Aikainterp,
    Mod,
    Pow,
    Aikavali,
    Abs,
    Log,
    Ln,
    Floor,
    Ceil,
    Sqrt,
    Exp,
    Interpoloi,
    Min,
    Max,
    Sum,
    Med,
    Kesk,
    If,
    SS,
    List,
    Pair,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    Empty,
    Minus,
    Plus,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Ast {
    Node(Vec<Ast>, Fun),
    Leaf(f64),
    Get(String),
    Empty,
}
impl From<Token> for Fun {
    fn from(token: Token) -> Self {
        return match token {
            Token::Abs => Fun::Abs,
            Token::Add => Fun::Add,
            Token::Aikainterp => Fun::Aikainterp,
            Token::Aikavali => Fun::Aikavali,
            Token::Arccos => Fun::Arccos,
            Token::Arcsin => Fun::Arcsin,
            Token::Arctan => Fun::Arctan,
            Token::Ceil => Fun::Ceil,
            Token::Cos => Fun::Cos,
            Token::Div => Fun::Div,
            Token::Eq => Fun::Eq,
            Token::Exp => Fun::Exp,
            Token::Floor => Fun::Floor,
            Token::Ge => Fun::Ge,
            Token::Gt => Fun::Gt,
            Token::If => Fun::If,
            Token::Imod | Token::Mod => Fun::Mod,
            Token::Interpoloi => Fun::Interpoloi,
            Token::Ipow | Token::Pow => Fun::Pow,
            Token::Kesk => Fun::Kesk,
            Token::Le => Fun::Le,
            Token::List => Fun::List,
            Token::Ln => Fun::Ln,
            Token::Log => Fun::Log,
            Token::Logb => Fun::Logb,
            Token::Lt => Fun::Lt,
            Token::Max => Fun::Max,
            Token::Min => Fun::Min,
            Token::Minus => Fun::Minus,
            Token::Mul => Fun::Mul,
            Token::Neq => Fun::Neq,
            Token::Plus => Fun::Plus,
            Token::SS => Fun::SS,
            Token::Sin => Fun::Sin,
            Token::Sqrt => Fun::Sqrt,
            Token::Sub => Fun::Sub,
            Token::Sum => Fun::Sum,
            Token::Tan => Fun::Cos,
            _ => Fun::Empty,
        };
    }
}


pub fn parse(input: Vec<Token>) -> Result<Ast, String> {
    parse_fn(input, applicators::basic, super::ctx::EmptyCtx)
}

/// We will use Shunting-Yard algorithm.
pub fn parse_fn<F, C: super::ctx::KilaCtx + Clone>(
    input: Vec<Token>,
    app: F,
    ctx: C,
) -> Result<Ast, String>
where
    F: Fn(Vec<Ast>, Fun, C) -> Ast,
{
    let mut prev: Vec<Token> = Vec::new();
    let mut opr: Vec<Token> = Vec::new();
    let mut node: Vec<Ast> = Vec::new();
    let mut arity: Vec<usize> = Vec::new();
    let mut iter = input.iter().cloned().peekable();
    while let Some(t) = iter.next() {
        match t.clone() {
            Token::Num(n) => node.push(Ast::Leaf(n)),
            Token::Expr(n) => node.push(Ast::Get(n)),
            Token::Empty => return Err(format!("Got empty")),
            Token::Comma => {
                *arity.last_mut().unwrap() += 1;
                while let Some(op) = opr.pop() {
                    match op {
                        Token::ParL => {
                            opr.push(op.clone());
                            break;
                        }
                        _ => {
                            let fun = Fun::from(op);
                            let ar = arity!(fun, arity);
                            let nod = app(children!(ar, node), fun, ctx.clone());
                            node.push(nod);
                        }
                    }
                }
            }
            Token::ParL => opr.push(Token::ParL),
            Token::ParR => {
                while let Some(op) = opr.pop() {
                    match op {
                        Token::ParL => break,
                        _ => {
                            let fun = Fun::from(op);
                            let ar = arity!(fun, arity);
                            let nod = app(children!(ar, node), fun, ctx.clone());
                            node.push(nod);
                        }
                    }
                }
            }
            Token::Eq | Token::Neq | Token::Gt | Token::Ge | Token::Lt | Token::Le |
            Token::Add | Token::Sub => {
                match prev.last() {
                    Some(pre) => {
                        match pre.clone() {
                            Token::ParR | Token::Num(_) | Token::Expr(_) => {
                                while let Some(op) = opr.pop() {
                                    match op {
                                        Token::ParL => {
                                            opr.push(Token::ParL);
                                            break;
                                        }
                                        _ => {
                                            let fun = Fun::from(op);
                                            let ar = arity!(fun, arity);
                                            let nod = app(children!(ar, node), fun, ctx.clone());
                                            node.push(nod);
                                        }
                                    }
                                }
                                opr.push(t.clone());
                            }
                            _ => {
                                opr.push(match t {
                                    Token::Add => Token::Plus,
                                    Token::Sub => Token::Minus,
                                    _ => return Err(format!("Shouldn't happen. Unary operator")),
                                })
                            }
                        }
                    }
                    None => {
                        opr.push(match t {
                            Token::Add => Token::Plus,
                            Token::Sub => Token::Minus,
                            _ => return Err(format!("Shouldn't happen. Unary operator")),
                        })
                    }
                }
            }
            Token::Mul | Token::Div | Token::Imod => {
                while let Some(op) = opr.pop() {
                    match op {
                        Token::Add | Token::Sub | Token::ParL => {
                            opr.push(op);
                            break;
                        }
                        _ => {
                            let fun = Fun::from(op);
                            let ar = arity!(fun, arity);
                            let nod = app(children!(ar, node), fun, ctx.clone());
                            node.push(nod);
                        }
                    }
                }
                opr.push(t.clone());
            }
            Token::Ipow => opr.push(Token::Ipow),
            _ => {
                opr.push(t.clone());
                arity.push(1);
            }
        }
        prev.push(t.clone());
    }
    while let Some(op) = opr.pop() {
        let fun = Fun::from(op);
        let ar = arity!(fun, arity);
        let nod = app(children!(ar, node), fun, ctx.clone());
        node.push(nod);
    }
    if node.len() > 1 {
        return Err(format!("Too many members: {:#?}", node));
    }
    return Ok(node.pop().unwrap());
}

#[cfg(test)]
mod tests {}
