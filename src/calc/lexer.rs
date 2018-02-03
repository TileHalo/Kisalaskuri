
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
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
    ParL,
    ParR,
    BrackL,
    BrackR,
    Comma,
    Expr(String),
    Num(f64),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pos: usize,
    inp: String,
}

impl Lexer {
    pub fn new(inp: &str) -> Lexer {
        Lexer {
            pos: 0,
            inp: String::from(inp),
        }
    }
    fn next_char(&self) -> char {
        self.inp[self.pos..].chars().next().unwrap()
    }
    fn starts_with(&self, s: &str) -> bool {
        self.inp[self.pos..].starts_with(s)
    }
    fn eof(&self) -> bool {
        self.pos >= self.inp.len()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.inp[self.pos..].char_indices();
        let (_, cur) = iter.next().unwrap();
        let (nxt, _) = iter.next().unwrap_or((1, ' '));
        self.pos += nxt;
        cur
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut res = String::new();
        while !self.eof() && test(self.next_char()) {
            res.push(self.consume_char());
        }
        res
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn get_expr(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric() || c == '.')
    }

    fn parse_expr(&mut self, expr: &str) -> Token {
        match expr {
            "aikavali" => Token::Aikavali,
            "abs" => Token::Abs,
            "log" => Token::Log,
            "ln" => Token::Ln,
            "floor" => Token::Floor,
            "ceil" => Token::Ceil,
            "sqrt" => Token::Sqrt,
            "exp" => Token::Exp,
            "mod" => Token::Mod,
            "pow" => Token::Pow,
            "interpoloi" => Token::Interpoloi,
            "min" | "pienin" => Token::Min,
            "max" | "suurin" => Token::Max,
            "sum" => Token::Sum,
            "med" => Token::Med,
            "kesk" | "mean" => Token::Kesk,
            "ss" => Token::SS,
            _ => {
                let num = expr.parse::<f64>();
                match num {
                    Ok(val) => Token::Num(val),
                    Err(_) => Token::Expr(String::from(expr)),
                }
            }
        }
    }
    pub fn lex(&mut self) -> Vec<Token> {
        let mut res: Vec<Token> = Vec::new();
        while !self.eof() {
            self.consume_whitespace();
            match self.next_char() {
                '(' => {
                    self.consume_char();
                    res.push(Token::ParL);
                }
                ')' => {
                    self.consume_char();
                    res.push(Token::ParR);
                }
                '[' => {
                    self.consume_char();
                    res.push(Token::BrackL);
                }
                ']' => {
                    self.consume_char();
                    res.push(Token::BrackR);
                }
                '+' => {
                    self.consume_char();
                    res.push(Token::Add);
                }
                '-' => {
                    self.consume_char();
                    res.push(Token::Sub);
                }
                '*' => {
                    self.consume_char();
                    res.push(Token::Mul);
                }
                '/' => {
                    self.consume_char();
                    res.push(Token::Div);
                }
                '^' => {
                    self.consume_char();
                    res.push(Token::Pow);
                }
                '%' => {
                    self.consume_char();
                    res.push(Token::Mod);
                }
                ',' => {
                    self.consume_char();
                    res.push(Token::Comma);
                }
                _ => {
                    let expr = self.get_expr();
                    res.push(self.parse_expr(&expr));
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_simple() {
        let mut lexer = Lexer::new("ss");
        assert_eq!(vec![Token::SS], lexer.lex());
    }
    #[test]
    fn test_function_kipa_interpolate() {
        let inp = "max(interpoloi(max([(a-0),0.5*med((.a*..mukana-0))]),
        max((.a*..mukana-0)),5,0.5*med((.a*..mukana-0))))";
        let res = vec![
            Token::Max,
            Token::ParL,
            Token::Interpoloi,
            Token::ParL,
            Token::Max,
            Token::ParL,
            Token::BrackL,
            Token::ParL,
            Token::Expr(String::from("a")),
            Token::Sub,
            Token::Num(0.0),
            Token::ParR,
            Token::Comma,
            Token::Num(0.5),
            Token::Mul,
            Token::Med,
            Token::ParL,
            Token::ParL,
            Token::Expr(String::from(".a")),
            Token::Mul,
            Token::Expr(String::from("..mukana")),
            Token::Sub,
            Token::Num(0.0),
            Token::ParR,
            Token::ParR,
            Token::BrackR,
            Token::ParR,
            Token::Comma,
            Token::Max,
            Token::ParL,
            Token::ParL,
            Token::Expr(String::from(".a")),
            Token::Mul,
            Token::Expr(String::from("..mukana")),
            Token::Sub,
            Token::Num(0.0),
            Token::ParR,
            Token::ParR,
            Token::Comma,
            Token::Num(5.0),
            Token::Comma,
            Token::Num(0.5),
            Token::Mul,
            Token::Med,
            Token::ParL,
            Token::ParL,
            Token::Expr(String::from(".a")),
            Token::Mul,
            Token::Expr(String::from("..mukana")),
            Token::Sub,
            Token::Num(0.0),
            Token::ParR,
            Token::ParR,
            Token::ParR,
            Token::ParR,
        ];
        let mut lexer = Lexer::new(inp);
        assert_eq!(res, lexer.lex());
    }
}
