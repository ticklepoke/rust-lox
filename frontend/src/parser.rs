use crate::ast::{Expr, Stmt};
use crate::literal::Literal;
use crate::token::{Token, TokenType};
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
        if self.match_token(vec![TokenType::Class]) {
            res = self.class_declaration();
        } else if self.match_token(vec![TokenType::Fun]) {
            res = self.function("function");
        } else if self.match_token(vec![TokenType::Var]) {
            res = self.var_declaration();
        } else {
            res = self.statement();
        }

        if res.is_err() {
            self.synchronize();
        }
        res
    }

    fn class_declaration(&mut self) -> ParserResult<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expected class name")?;
        self.consume(TokenType::LeftBrace, "Expected '{' before class body")?;

        let mut methods = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_end() {
            methods.push(self.function("method")?);
        }
        self.consume(TokenType::RightBrace, "Expected '}' after class body")?;

        Ok(Stmt::Class(name, methods))
    }

    fn var_declaration(&mut self) -> ParserResult<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expected variable name")?;

        let mut init = None;
        if self.match_token(vec![TokenType::Equal]) {
            init = Some(self.expression()?);
        }

        self.consume(
            TokenType::SemiColon,
            "expected ';' after variable declaration",
        )?;
        Ok(Stmt::Var(name, init))
    }

    fn function(&mut self, kind: &str) -> ParserResult<Stmt> {
        let name = self.consume(
            TokenType::Identifier,
            format!("Expect {} kind", kind).as_str(),
        )?;
        self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {} name", kind).as_str(),
        )?;
        let mut params = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(ParserError::ArgumentCountExceeded);
                }
                params.push(self.consume(TokenType::Identifier, "Expect param name")?);

                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters")?;

        self.consume(
            TokenType::LeftBrace,
            format!("Expect '{{' before {} body", kind).as_str(),
        )?;

        let body = self.block()?;

        Ok(Stmt::Function(name, params, body))
    }

    fn statement(&mut self) -> ParserResult<Stmt> {
        if self.match_token(vec![TokenType::For]) {
            return self.for_statement();
        }
        if self.match_token(vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.match_token(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token(vec![TokenType::Return]) {
            return self.return_statement();
        }
        if self.match_token(vec![TokenType::While]) {
            return self.while_statement();
        }
        if self.match_token(vec![TokenType::LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        self.expression_statement()
    }

    fn for_statement(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'")?;
        let init;
        if self.match_token(vec![TokenType::SemiColon]) {
            init = None;
        } else if self.match_token(vec![TokenType::Var]) {
            init = Some(self.var_declaration()?);
        } else {
            init = Some(self.expression_statement()?);
        }

        let mut condition = None;
        if !self.check(TokenType::SemiColon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::SemiColon, "Expect ';' after loop condition")?;

        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clause")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block(vec![body, Stmt::Expr(increment)]);
        }

        if condition.is_none() {
            condition = Some(Expr::Literal(Literal::Boolean(true)));
        }
        body = Stmt::While(condition.unwrap(), Box::new(body));

        if let Some(init) = init {
            body = Stmt::Block(vec![init, body]);
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition")?;
        let body = self.statement()?;
        Ok(Stmt::While(condition, Box::new(body)))
    }

    fn if_statement(&mut self) -> ParserResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition")?;
        let consequent = self.statement()?;
        let mut alternative = None;
        if self.match_token(vec![TokenType::Else]) {
            alternative = Some(Box::new(self.statement()?));
        }
        Ok(Stmt::If(condition, Box::new(consequent), alternative))
    }

    fn return_statement(&mut self) -> ParserResult<Stmt> {
        let keyword = self.previous().clone();
        let mut value = None;
        if !self.check(TokenType::SemiColon) {
            value = Some(self.expression()?);
        }
        self.consume(TokenType::SemiColon, "Expect ';' after return value")?;
        Ok(Stmt::Return(keyword, value))
    }

    fn block(&mut self) -> ParserResult<Vec<Stmt>> {
        let mut stmts = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_end() {
            stmts.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expected '}' after block")?;
        Ok(stmts)
    }

    fn print_statement(&mut self) -> ParserResult<Stmt> {
        let val = self.expression()?;
        self.consume(TokenType::SemiColon, "Expected ';' after print statement")?;
        Ok(Stmt::Print(val))
    }

    fn expression_statement(&mut self) -> ParserResult<Stmt> {
        let val = self.expression()?;
        self.consume(TokenType::SemiColon, "Expected ';' after print statement")?;
        Ok(Stmt::Expr(val))
    }

    fn expression(&mut self) -> ParserResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParserResult<Expr> {
        let expr = self.or()?;

        if self.match_token(vec![TokenType::Equal]) {
            let value = self.assignment()?;

            return match expr {
                Expr::Variable(name) => Ok(Expr::Assign(name, Box::new(value))),
                Expr::Get(obj, field_name) => Ok(Expr::Set(obj, field_name, Box::new(value))),
                _ => Err(ParserError::InvalidAssignmentTarget),
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> ParserResult<Expr> {
        let mut expr = self.and()?;
        while self.match_token(vec![TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn and(&mut self) -> ParserResult<Expr> {
        let mut expr = self.equality()?;
        while self.match_token(vec![TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
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
        self.call()
    }

    fn call(&mut self) -> ParserResult<Expr> {
        let mut expr = self.primary()?;
        loop {
            if self.match_token(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(vec![TokenType::Dot]) {
                let name =
                    self.consume(TokenType::Identifier, "Expected property name after '.'")?;
                expr = Expr::Get(Box::new(expr), name);
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> ParserResult<Expr> {
        let mut args = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if args.len() >= 255 {
                    return Err(ParserError::ArgumentCountExceeded);
                }
                args.push(self.expression()?);
                if !self.match_token(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments")?;

        Ok(Expr::Call(Box::new(callee), paren, args))
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
    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, ParserError> {
        if self.check(token_type) {
            Ok(self.advance().clone())
        } else {
            Err(ParserError::GenericError(msg.to_string(), self.peek().line))
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
