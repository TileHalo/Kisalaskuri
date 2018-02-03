use super::lexer::{Token, Lexer};
macro_rules! recurse {
    ($e:ident, $i:ident, $n:ident) => (
        {
            let (res, mut inp) = parse_internal($i.clone(), Fun::$e);
            $i = inp;
            $n.push(Box::new(res));
        }
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
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Ast {
    Node(Vec<Box<Ast>>, Fun),
    Leaf(f64),
    Get(String),
    Empty,
}


fn parse_internal(mut input: Vec<Token>, cur: Fun) -> (Ast, Vec<Token>) {
    let mut nodes: Vec<Box<Ast>> = Vec::new();
    while let Some(token) = input.pop() {
        match token {
            Token::Add => recurse!(Add, input, nodes),
            Token::Sub => recurse!(Sub, input, nodes),
            Token::Div => recurse!(Div, input, nodes),
            Token::Mul => recurse!(Mul, input, nodes),
            Token::Mod => recurse!(Mod, input, nodes),
            Token::Pow => recurse!(Pow, input, nodes),
            Token::Sum => recurse!(Sum, input, nodes),
            Token::Aikavali => recurse!(Aikavali, input, nodes),
            Token::Abs => recurse!(Abs, input, nodes),
            Token::Log => recurse!(Log, input, nodes),
            Token::Ln => recurse!(Ln, input, nodes),
            Token::Floor => recurse!(Floor, input, nodes),
            Token::Ceil => recurse!(Ceil, input, nodes),
            Token::Sqrt => recurse!(Sqrt, input, nodes),
            Token::Exp => recurse!(Exp, input, nodes),
            Token::Interpoloi => recurse!(Interpoloi, input, nodes),
            Token::Min => recurse!(Min, input, nodes),
            Token::Max => recurse!(Max, input, nodes),
            Token::Med => recurse!(Med, input, nodes),
            Token::Kesk => recurse!(Kesk, input, nodes),
            Token::If => recurse!(If, input, nodes),
            Token::SS => unimplemented!(),
            Token::ParL => (),
            Token::ParR => return (Ast::Node(nodes, cur), input),
            Token::BrackL => recurse!(List, input, nodes),
            Token::BrackR => return (Ast::Node(nodes, cur), input),
            Token::Comma => {
                match cur {
                    Fun::Add | Fun::Sub | Fun::Div | Fun::Mul | Fun::Mod | Fun::Pow => {
                        return (Ast::Node(nodes, cur), input)
                    }
                    _ => (),
                }
            }
            Token::Num(num) => nodes.push(Box::new(Ast::Leaf(num))),
            Token::Expr(expr) => nodes.push(Box::new(Ast::Get(expr))),
            _ => panic!("Shiet"),
        }
    }
    return (Ast::Node(nodes, cur), input);
}

pub fn parse(mut input: Vec<Token>) -> Ast {
    input.reverse();
    let mut cur = Fun::Empty;
    match input.pop().unwrap() {
        Token::Add => cur = Fun::Add,
        Token::Sub => cur = Fun::Sub,
        Token::Sum => cur = Fun::Sum,
        Token::Div => cur = Fun::Div,
        Token::Mul => cur = Fun::Mul,
        Token::Mod => cur = Fun::Mod,
        Token::Pow => cur = Fun::Pow,
        Token::Aikavali => cur = Fun::Aikavali,
        Token::Abs => cur = Fun::Abs,
        Token::Log => cur = Fun::Log,
        Token::Ln => cur = Fun::Ln,
        Token::Floor => cur = Fun::Floor,
        Token::Ceil => cur = Fun::Ceil,
        Token::Sqrt => cur = Fun::Sqrt,
        Token::Exp => cur = Fun::Exp,
        Token::Interpoloi => cur = Fun::Interpoloi,
        Token::Min => cur = Fun::Min,
        Token::Max => cur = Fun::Max,
        Token::Med => cur = Fun::Med,
        Token::Kesk => cur = Fun::Kesk,
        Token::If => cur = Fun::If,
        Token::SS => unimplemented!(),
        Token::Num(num) => input.push(Token::Num(num)),
        Token::Expr(expr) => input.push(Token::Expr(expr)),
        _ => panic!("Shiet"),
    }
    parse_internal(input, cur).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let tokens = vec![
            Token::Min,
            Token::ParL,
            Token::Num(0.0),
            Token::Num(5.0),
            Token::ParR,
        ];
        let ast = Ast::Node(
            vec![Box::new(Ast::Leaf(0.0)), Box::new(Ast::Leaf(5.0))],
            Fun::Min,
        );
        assert_eq!(ast, parse(tokens));
    }

    #[test]
    fn test_parse_nest() {
        // min(0.0, min(5, 6))
        let tokens = vec![
            Token::Min,
            Token::ParL,
            Token::Num(0.0),
            Token::Min,
            Token::ParL,
            Token::Num(5.0),
            Token::Num(6.0),
            Token::ParR,
            Token::ParR,
            Token::ParR,
        ];
        let res = Ast::Node(
            vec![
                Box::new(Ast::Leaf(0.0)),
                Box::new(Ast::Node(
                    vec![Box::new(Ast::Leaf(5.0)), Box::new(Ast::Leaf(6.0))],
                    Fun::Min,
                )),
            ],
            Fun::Min,
        );
        assert_eq!(res, parse(tokens));
    }

    #[test]
    fn test_lexparse() {
        let mut lexer = Lexer::new("min(0.0, 5.0, 6.0)");
        let lexed = lexer.lex();
        assert_eq!(
            Ast::Node(
                vec![
                    Box::new(Ast::Leaf(0.0)),
                    Box::new(Ast::Leaf(5.0)),
                    Box::new(Ast::Leaf(6.0)),
                ],
                Fun::Min,
            ),
            parse(lexed)
        );
    }
    #[test]
    fn test_sqrt() {
        let mut lexer = Lexer::new("sqrt(5.5)");
        let lexed = lexer.lex();
        assert_eq!(
            Ast::Node(vec![Box::new(Ast::Leaf(5.5))], Fun::Sqrt),
            parse(lexed)
        );
    }
    #[test]
    fn test_add() {
        let mut lexer = Lexer::new("+ 5 6");
        assert_eq!(
            Ast::Node(
                vec![Box::new(Ast::Leaf(5.0)), Box::new(Ast::Leaf(6.0))],
                Fun::Add,
            ),
            parse(lexer.lex())
        );
    }
}
