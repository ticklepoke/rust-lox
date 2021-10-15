use crate::ast::Expr;
use lexer::literal::Literal;
use lexer::token::{Token, TokenType};
use utils::errors::ParserError;

pub type ParserResult = Result<Expr, ParserError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParserResult {
        self.expression()
    }

    // AST NODE Fns
    fn expression(&mut self) -> ParserResult {
        self.equality()
    }

    fn equality(&mut self) -> ParserResult {
        let mut expr = self.comparison()?;
        use TokenType::*;
        while self.match_token(vec![BangEqual, EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ParserResult {
        let mut expr = self.term()?;

        use TokenType::*;
        while self.match_token(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }

    fn term(&mut self) -> ParserResult {
        let mut expr = self.factor()?;

        use TokenType::*;
        while self.match_token(vec![Minus, Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ParserResult {
        let mut expr = self.unary()?;

        use TokenType::*;
        while self.match_token(vec![Slash, Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParserResult {
        use TokenType::*;
        if self.match_token(vec![Bang, Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> ParserResult {
        use TokenType::*;
        if self.match_token(vec![False]) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }
        if self.match_token(vec![True]) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }
        if self.match_token(vec![Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if self.match_token(vec![Number]) {
            if let Literal::Number(f) = self.previous().literal.as_ref().unwrap() {
                return Ok(Expr::Literal(Literal::Number(*f)));
            }
            // TODO dont unwrap early?
        }
        if self.match_token(vec![String]) {
            if let Literal::String(s) = self.previous().literal.as_ref().unwrap() {
                return Ok(Expr::Literal(Literal::String(s.to_string())));
            }
        }
        if self.match_token(vec![LeftParen]) {
            let expr = self.expression()?;
            //self.con
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        Err(ParserError::UnexpectedToken(
            "Expected expression".to_string(),
            self.peek().line,
        ))
    }

    // MISC UTILS FNs
    fn match_token(&mut self, token_types: Vec<TokenType>) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .expect("Token index out of bounds")
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("Token index out of bounds")
    }

    // Error Utils
    fn consume(&mut self, token_type: TokenType, msg: String) -> Result<Token, ParserError> {
        if self.check(token_type) {
            Ok(self.advance().clone())
        } else {
            Err(ParserError::GenericError(msg, self.peek().line))
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_end() {
            use TokenType::*;
            if self.previous().token_type == SemiColon {
                return;
            }

            match self.peek().token_type {
                Class | For | Fun | If | Print | Return | Var | While => return,
                _ => (),
            };

            self.advance();
        }
    }
}
