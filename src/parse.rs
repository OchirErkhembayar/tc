use std::{collections::HashMap, fmt::Display};

use crate::{inner_write, token::Token};
const COS: &str = "cos";
const SIN: &str = "sin";
const TAN: &str = "tan";
const LOG: &str = "log";
const LN: &str = "ln";

pub const FUNCS: [&str; 5] = [COS, SIN, TAN, LOG, LN];

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    values: HashMap<char, f64>,
}

#[derive(Debug, PartialEq)]
pub struct ParseErr {
    pub token: Token,
    pub msg: &'static str,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Func {
    Sin,
    Cos,
    Tan,
    Ln,
    Log(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Num(f64),
    Negative(Box<Expr>),
    Abs(Box<Expr>),
    Exponent(Box<Expr>, Box<Expr>),
    Func(Func, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Func::Sin => SIN,
                Func::Cos => COS,
                Func::Tan => TAN,
                Func::Ln => LN,
                Func::Log(base) => return inner_write(format!("log({})", base), f),
            }
        )
    }
}

impl Expr {
    pub fn format(&self) -> String {
        match self {
            Self::Num(num) => num.to_string(),
            Self::Negative(expr) => format!("-{}", expr.format()),
            Self::Grouping(expr) => format!("({})", expr.format()),
            Self::Abs(expr) => format!("|{}|", expr.format()),
            Self::Binary(left, operator, right) => {
                format!("{}{}{}", left.format(), operator, right.format())
            }
            Self::Exponent(base, exponent) => format!("{}^{}", base.format(), exponent.format()),
            Self::Func(func, argument) => format!("{}({})", func, argument.format()),
        }
    }
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERROR: {}", self.msg)
    }
}

impl ParseErr {
    pub fn new(token: Token, msg: &'static str) -> Self {
        Self { token, msg }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>, values: HashMap<char, f64>) -> Self {
        Self {
            tokens,
            current: 0,
            values,
        }
    }

    fn at_end(&self) -> bool {
        self.peek() == Token::Eoe
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn check(&self, token: Token) -> bool {
        if self.at_end() {
            return false;
        }
        self.peek() == token
    }

    fn consume(&mut self, token: Token, msg: &'static str) -> Result<Token, ParseErr> {
        if self.check(token) {
            Ok(self.advance())
        } else {
            Err(ParseErr::new(token, msg))
        }
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<Expr, ParseErr> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParseErr> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.factor()?;
        while self.peek() == Token::Plus || self.peek() == Token::Minus {
            let operator = self.advance();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.exponent()?;
        while self.peek() == Token::Div || self.peek() == Token::Mult || self.peek() == Token::Mod {
            let operator = self.advance();
            let right = self.exponent()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn exponent(&mut self) -> Result<Expr, ParseErr> {
        let mut expr = self.negative()?;
        while self.peek() == Token::Power {
            self.advance();
            let right = self.negative()?;
            expr = Expr::Exponent(Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    fn negative(&mut self) -> Result<Expr, ParseErr> {
        if self.peek() == Token::Minus {
            self.advance();
            let right = self.negative()?;
            Ok(Expr::Negative(Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseErr> {
        let token = self.peek();
        if let Token::Num(num) = token {
            self.advance();
            return Ok(Expr::Num(num));
        }
        if let Token::LParen = token {
            self.advance();
            let expr = Box::new(self.expression()?);
            self.consume(Token::RParen, "Missing closing parentheses")?;
            return Ok(Expr::Grouping(expr));
        }
        if let Token::Pipe = token {
            self.advance();
            let expr = Box::new(self.expression()?);
            self.consume(Token::Pipe, "Missing closing pipe")?;
            return Ok(Expr::Abs(expr));
        }
        if let Token::Var(var) = token {
            if let Some(&num) = self.values.get(&var) {
                self.advance();
                return Ok(Expr::Num(num));
            } else {
                return Err(ParseErr::new(token, "Unknown variable"));
            }
        }
        if let Token::Func(func) = token {
            self.advance();
            let func = match func {
                SIN => Func::Sin,
                COS => Func::Cos,
                TAN => Func::Tan,
                LN => Func::Ln,
                LOG => {
                    if let Token::Num(base) = self.advance() {
                        Func::Log(base)
                    } else {
                        return Err(ParseErr::new(token, "Missing base for log function"));
                    }
                }
                _ => return Err(ParseErr::new(token, "Unknown function")),
            };
            self.consume(Token::LParen, "Missing opening parentheses")?;
            let arg = Box::new(self.expression()?);
            self.consume(Token::RParen, "Missing closing parentheses")?;
            return Ok(Expr::Func(func, arg));
        }
        Err(ParseErr::new(token, "Expected expression"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(tokens: Vec<Token>, expected: Expr) {
        let mut parser = Parser::new(tokens, HashMap::new());
        let expr = parser.parse().unwrap();

        assert_eq!(expr, expected,);
    }

    #[test]
    fn simple_add() {
        let tokens = vec![Token::Num(10.0), Token::Plus, Token::Num(5.0), Token::Eoe];
        let expr = Expr::Binary(
            Box::new(Expr::Num(10.0)),
            Token::Plus,
            Box::new(Expr::Num(5.0)),
        );

        check(tokens, expr);
    }

    #[test]
    fn simple_mult() {
        let tokens = vec![Token::Num(20.0), Token::Mult, Token::Num(5.0), Token::Eoe];
        let expr = Expr::Binary(
            Box::new(Expr::Num(20.0)),
            Token::Mult,
            Box::new(Expr::Num(5.0)),
        );

        check(tokens, expr);
    }

    #[test]
    fn grouping() {
        let tokens = vec![
            Token::LParen,
            Token::Num(1.0),
            Token::Plus,
            Token::Num(2.0),
            Token::RParen,
            Token::Mult,
            Token::Num(5.0),
            Token::Eoe,
        ];
        let expr = Expr::Binary(
            Box::new(Expr::Grouping(Box::new(Expr::Binary(
                Box::new(Expr::Num(1.0)),
                Token::Plus,
                Box::new(Expr::Num(2.0)),
            )))),
            Token::Mult,
            Box::new(Expr::Num(5.0)),
        );

        check(tokens, expr);
    }

    #[test]
    fn negative() {
        let tokens = vec![
            Token::Minus,
            Token::LParen,
            Token::Num(5.0),
            Token::RParen,
            Token::Div,
            Token::Num(1.0),
            Token::Eoe,
        ];
        let expr = Expr::Binary(
            Box::new(Expr::Negative(Box::new(Expr::Grouping(Box::new(
                Expr::Num(5.0),
            ))))),
            Token::Div,
            Box::new(Expr::Num(1.0)),
        );

        check(tokens, expr);
    }

    #[test]
    fn test_missing_closing_paren() {
        let tokens = vec![Token::Minus, Token::LParen, Token::Num(5.0), Token::Eoe];
        if let Err(err) = Parser::new(tokens, HashMap::new()).parse() {
            assert_eq!(
                err,
                ParseErr::new(Token::RParen, "Missing closing parentheses")
            );
        } else {
            panic!("Didn't return error");
        }
    }

    #[test]
    fn test_variable() {
        let tokens = vec![Token::Var('a'), Token::Plus, Token::Num(3.0), Token::Eoe];
        let mut parser = Parser::new(tokens, HashMap::from_iter([('a', 1.0)]));
        let expr = Expr::Binary(
            Box::new(Expr::Num(1.0)),
            Token::Div,
            Box::new(Expr::Num(3.0)),
        );

        let expected = parser.parse().unwrap();

        assert_eq!(expected, expr);
    }
}