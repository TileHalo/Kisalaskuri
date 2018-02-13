use super::lexer::Token;
// macro_rules! recurse {
//     ($e:ident, $i:ident, $n:ident) => (
//         {
//             let (res, mut inp) = parse_internal($i.clone(), Fun::$e);
//             $i = inp;
//             $n.push(Box::new(res));
//         }
//     )
// }

// macro_rules! infix_recurse {
//     ($e:ident, $i:ident, $n:ident) => (
//         {
//             let (res, mut inp) = parse_internal($i.clone(), Fun::$e);
//             $i = inp;
//             $n.push(Box::new(res));
//         }
//     )
// }

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
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Ast {
    Node(Vec<Ast>, Fun),
    Leaf(f64),
    Get(String),
    Empty,
}

/// We will use Shunting-Yard algorithm.
/// Pseudocode:
///
/// ```text
/// while there are tokens to be read:
///     read a token.
///     if the token is a number, then push it to the output queue.
///     if the token is an operator, then:
/// 	    while ((there is an operator at the top of the operator stack with
/// 		    greater precedence) or (the operator at the top of the operator stack has
///                 equal precedence and
///                 the operator is left associative)) and
///                 (the operator at the top of the stack is not a left bracket):
/// 			pop operators from the operator stack, onto the output queue.
/// 	push the read operator onto the operator stack.
/// if the token is a left bracket (i.e. "("), then:
/// 	push it onto the operator stack.
/// if the token is a right bracket (i.e. ")"), then:
/// 	while the operator at the top of the operator stack is not a left bracket:
/// 		pop operators from the operator stack onto the output queue.
/// 	pop the left bracket from the stack.
/// 	/* if the stack runs out without finding a left bracket, then there are
/// 	mismatched parentheses. */
/// if there are no more tokens to read:
/// while there are still operator tokens on the stack:
/// 	/* if the operator token on the top of the stack is a bracket, then
/// 	there are mismatched parentheses. */
/// 	pop the operator onto the output queue.
/// exit.
/// ```
///
///with these modifications:
///
///```text
/// The shunting yard algorithm can be used to build an AST.
/// You need an additional stack of tree nodes (this is different from the operator stack),
/// which is intially empty. When you would output an operand, you instead
/// create a leaf node from it and push that onto the stack. Whenever you
/// would output an operator, you create a node from it, then pop the two top
/// operands from the output stack (or one if it's an unary operator), and add
/// them as child nodes. Then you push the resulting tree to the output stack.
///```
pub fn parse(input: Vec<Token>) -> Ast {
    let mut opr: Vec<Token> = Vec::new();
    let mut node: Vec<Ast> = Vec::new();
    let mut iter = input.iter().cloned();
    while let Some(t) = iter.next() {
        match t {
            Token::Num(n) => node.push(Ast::Leaf(n)),
            Token::Expr(n) => node.push(Ast::Get(n)),
            Token::Empty => panic!("Got empty"),
            Token::Comma => unimplemented!(),
            Token::ParL => unimplemented!(),
            Token::ParR => unimplemented!(),
            Token::BrackL => unimplemented!(),
            Token::BrackR => unimplemented!(),
            Token::Add | Token::Sub => {
                while let Some(oper) = opr.pop() {
                    match oper {
                        _ => {
                            let fun = match oper {
                                Token::Add => Fun::Add,
                                Token::Sub => Fun::Sub,
                                _ => panic!("wrong token {:#?}", oper)
                            };
                            let nod = Ast::Node(
                                vec![node.pop().unwrap(), node.pop().unwrap()],
                                fun,
                            );
                            node.push(nod);
                        }
                    }
                }
                opr.push(t.clone());
            },
            Token::Mul | Token::Div => {
                while let Some(oper) = opr.pop() {
                    match oper {
                        Token::Add | Token::Sub => {
                            opr.push(oper);
                            break;
                        },
                        _ => {
                            let fun = match oper {
                                Token::Mul => Fun::Mul,
                                Token::Div => Fun::Div,
                                Token::Ipow => Fun::Pow,
                                _ => panic!("wrong token {:#?}", oper)
                            };
                            let nod = Ast::Node(
                                vec![node.pop().unwrap(), node.pop().unwrap()],
                                fun
                            );
                            node.push(nod);

                        }
                    }
                }
                opr.push(t.clone());
            }
            Token::Ipow => opr.push(Token::Ipow),
            _ => unimplemented!(),
        }
    }
    println!("{:#?}", opr);
    println!("{:#?}", node);
    while let Some(op) = opr.pop() {
        let fun = match op{
            Token::Add => Fun::Add,
            Token::Sub => Fun::Sub,
            Token::Mul => Fun::Mul,
            Token::Div => Fun::Div,
            Token::Ipow => Fun::Pow,
            _ => panic!("Invalid operator: {:#?}", op)
        };
        let nod = Ast::Node(vec![node.pop().unwrap(), node.pop().unwrap()], fun);
        node.push(nod);
    }
    return node.pop().unwrap();
}

#[cfg(test)]
mod tests {
}
