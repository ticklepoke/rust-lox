use crate::ast::Expr;
use crate::error::{ParserError, ParserResult};
use lexer::token::{Literal, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    // AST NODE Fns
    fn expression(&mut self) -> ParserResult {
        self.equality()
    }

    fn equality(&mut self) -> ParserResult {
        let mut expr = self.comparison()?;
        use TokenType::*;
        while self.match_token(vec![BangEqual, EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ParserResult {
        let mut expr = self.term()?;

        use TokenType::*;
        while self.match_token(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = *self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }

    fn term(&mut self) -> ParserResult {
        let mut expr = self.factor()?;

        use TokenType::*;
        while self.match_token(vec![Minus, Plus]) {
            let operator = *self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ParserResult {
        let mut expr = self.unary()?;

        use TokenType::*;
        while self.match_token(vec![Slash, Star]) {
            let operator = *self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParserResult {
        use TokenType::*;
        if self.match_token(vec![Bang, Minus]) {
            let operator = *self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> ParserResult {
        use TokenType::*;
        if self.match_token(vec![False]) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.match_token(vec![True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.match_token(vec![Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }
        if self.match_token(vec![Number]) {
            if let Literal::Float(f) = self.previous().literal.as_ref().unwrap() {
                return Ok(Expr::Literal(Literal::Float(*f)));
            } // TODO dont unwrap early?
        }
        if self.match_token(vec![String]) {
            if let Literal::Str(s) = self.previous().literal.as_ref().unwrap() {
                return Ok(Expr::Literal(Literal::Str(s.to_string())));
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
    fn match_token(&mut self, tokens: Vec<TokenType>) -> bool {
        for t in tokens {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_end() {
            return false;
        }
        self.peek().token_type == token
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
}
