use super::lexer::Token;

macro_rules! arity {
    ($e:expr, $b:expr) => (
        match $e {
            Fun::Add | Fun::Sub | Fun::Div | Fun::Mul | Fun::Mod | Fun::Pow => 2,
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
            v
        };
    )
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Fun {
    Add,
    Sub,
    Div,
    Mul,
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
            Token::Aikavali => Fun::Aikavali,
            Token::Ceil => Fun::Ceil,
            Token::Div => Fun::Div,
            Token::Exp => Fun::Exp,
            Token::Floor => Fun::Floor,
            Token::If => Fun::If,
            Token::Imod | Token::Mod => Fun::Mod,
            Token::Interpoloi => Fun::Interpoloi,
            Token::Ipow | Token::Pow => Fun::Pow,
            Token::Kesk => Fun::Kesk,
            Token::Ln => Fun::Ln,
            Token::Log => Fun::Log,
            Token::Max => Fun::Max,
            Token::Min => Fun::Min,
            Token::Mul => Fun::Mul,
            Token::Sqrt => Fun::Sqrt,
            Token::Sub => Fun::Sub,
            Token::Sum => Fun::Sum,
            Token::SS => Fun::SS,
            Token::List => Fun::List,
            Token::Plus => Fun::Plus,
            Token::Minus => Fun::Minus,
            _ => Fun::Empty
        };
    }
}

/// We will use Shunting-Yard algorithm.
/// Pseudocode:
pub fn parse(input: Vec<Token>) -> Ast {
    let mut prev: Vec<Token> = Vec::new();
    let mut opr: Vec<Token> = Vec::new();
    let mut node: Vec<Ast> = Vec::new();
    let mut arity: Vec<usize> = Vec::new();
    let mut iter = input.iter().cloned().peekable();
    while let Some(t) = iter.next() {
        match t.clone() {
            Token::Num(n) => node.push(Ast::Leaf(n)),
            Token::Expr(n) => node.push(Ast::Get(n)),
            Token::Empty => panic!("Got empty"),
            Token::Comma => {
                *arity.last_mut().unwrap() += 1; 
            },
            Token::ParL => {
                opr.push(Token::ParL);
            }
            Token::ParR => {
                while let Some(op) = opr.pop() {
                    match op {
                        Token::ParL => break,
                        _ => {
                            let fun = Fun::from(op);
                            let ar = arity!(fun, arity);
                            let nod =
                                Ast::Node(children!(ar, node), fun);
                            node.push(nod);
                        }
                    }
                }
            }
            Token::BrackL => unimplemented!(),
            Token::BrackR => unimplemented!(),
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
                                            let nod =
                                                Ast::Node(children!(ar, node), fun);
                                            node.push(nod);
                                        }
                                    }
                                }
                                opr.push(t.clone());
                            },
                            _ => opr.push(match t {
                                Token::Add => Token::Plus,
                                Token::Sub => Token::Minus,
                                _ => panic!("Shouldn't happen. Unary operator")
                            })
                        }
                    },
                    None => opr.push(match t {
                                Token::Add => Token::Plus,
                                Token::Sub => Token::Minus,
                                _ => panic!("Shouldn't happen. Unary operator")
                            })
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
                            let nod =
                                Ast::Node(children!(ar, node), fun);
                            node.push(nod);
                        }
                    }
                }
                opr.push(t.clone());
            }
            Token::Ipow => opr.push(Token::Ipow),
            _ => { opr.push(t.clone()); arity.push(1);},
        }
        prev.push(t.clone());
    }
    while let Some(op) = opr.pop() {
        let fun = Fun::from(op);
                            let ar = arity!(fun, arity);
        let nod = Ast::Node(children!(ar, node), fun);
        node.push(nod);
    }
    return node.pop().unwrap();
}

#[cfg(test)]
mod tests {
}
