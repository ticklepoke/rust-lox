use crate::ast::{Expr, Stmt};
use lexer::literal::Literal;
use lexer::token::{Token, TokenType};
use utils::errors::ParserError;

pub type ParserResult<T> = Result<T, ParserError>;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParserResult<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.is_end() {
            if let Ok(decl) = self.declaration() {
                statements.push(decl);
            }
        }
        Ok(statements)
    }

    // AST NODE Fns
    fn declaration(&mut self) -> ParserResult<Stmt> {
        let res;
        if self.match_token(vec![TokenType::Var]) {
            res = self.var_declaration()
        } else {
            res = self.statement()
        }

        if res.is_err() {
            self.synchronize();
        }
        res
    }

    fn var_declaration(&mut self) -> ParserResult<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expected variable name".to_string())?;

        let mut init = None;
        if self.match_token(vec![TokenType::Equal]) {
            init = Some(self.expression()?);
        }

        self.consume(
            TokenType::SemiColon,
            "expected ';' after variable declaration".to_string(),
        )?;
        Ok(Stmt::Var(name, init))
    }

    fn statement(&mut self) -> ParserResult<Stmt> {
        if self.match_token(vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token(vec![TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        self.expression_statement()
    }

    fn if_statement(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'".to_string())?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after if condition".to_string(),
        )?;
        let consequent = self.statement()?;
        let mut alternative = None;
        if self.match_token(vec![TokenType::Else]) {
            alternative = Some(Box::new(self.statement()?));
        }
        Ok(Stmt::If(condition, Box::new(consequent), alternative))
    }

    fn block(&mut self) -> ParserResult<Vec<Stmt>> {
        let mut stmts = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_end() {
            stmts.push(self.declaration()?);
        }

        self.consume(
            TokenType::RightBrace,
            "Expected '}' after block".to_string(),
        )?;
        Ok(stmts)
    }

    fn print_statement(&mut self) -> ParserResult<Stmt> {
        let val = self.expression()?;
        self.consume(
            TokenType::SemiColon,
            "Expected ';' after print statement".to_string(),
        )?;
        Ok(Stmt::Print(val))
    }

    fn expression_statement(&mut self) -> ParserResult<Stmt> {
        let val = self.expression()?;
        self.consume(
            TokenType::SemiColon,
            "Expected ';' after print statement".to_string(),
        )?;
        Ok(Stmt::Expr(val))
    }

    fn expression(&mut self) -> ParserResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParserResult<Expr> {
        let expr = self.equality()?;

        if self.match_token(vec![TokenType::Equal]) {
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            }

            return Err(ParserError::InvalidAssignmentTarget);
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ParserResult<Expr> {
        let mut expr = self.comparison()?;
        use TokenType::*;
        while self.match_token(vec![BangEqual, EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ParserResult<Expr> {
        let mut expr = self.term()?;

        use TokenType::*;
        while self.match_token(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }
        Ok(expr)
    }

    fn term(&mut self) -> ParserResult<Expr> {
        let mut expr = self.factor()?;

        use TokenType::*;
        while self.match_token(vec![Minus, Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> ParserResult<Expr> {
        let mut expr = self.unary()?;

        use TokenType::*;
        while self.match_token(vec![Slash, Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParserResult<Expr> {
        use TokenType::*;
        if self.match_token(vec![Bang, Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> ParserResult<Expr> {
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
        if self.match_token(vec![Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
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
