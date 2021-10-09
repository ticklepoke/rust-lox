use std::collections::HashMap;
use std::{fmt, iter, str};

use crate::token::{Literal, Token, TokenType};

/*
 * TODO: Block style comments
 */

type ScannerResult<T> = Result<T, ScannerError>;

#[derive(Debug)]
pub enum ScannerError {
    UnknownCharacter(char, usize),
    UntermiantedString(usize),
    InvalidCharacter(char, usize),
    InvalidTerm(String, usize),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ScannerError::UnknownCharacter(c, line_number) => {
                write!(f, "Unrecognised character {} at line {}", c, line_number)
            }
            ScannerError::UntermiantedString(line_number) => {
                write!(f, "Unterminated string at line {}", line_number)
            }
            ScannerError::InvalidCharacter(c, line_number) => {
                write!(f, "Invalid character {} at line {}", c, line_number)
            }
            ScannerError::InvalidTerm(s, line_number) => {
                write!(f, "Invalid term {} at line {}", s.clone(), line_number)
            }
        }
    }
}

impl ScannerError {
    pub fn line(&self) -> usize {
        match *self {
            ScannerError::UnknownCharacter(_, line_number) => line_number,
            ScannerError::UntermiantedString(line_number) => line_number,
            ScannerError::InvalidCharacter(_, line_number) => line_number,
            ScannerError::InvalidTerm(_, line_number) => line_number,
        }
    }
}

pub struct Scanner<'a> {
    source: iter::Peekable<str::Chars<'a>>,
    start: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut keywords: HashMap<&str, TokenType> = HashMap::new();
        use crate::token::TokenType::*;
        keywords.insert("and", And);
        keywords.insert("class", Class);
        keywords.insert("else", Else);
        keywords.insert("false", False);
        keywords.insert("for", For);
        keywords.insert("fun", Fun);
        keywords.insert("if", If);
        keywords.insert("nil", Nil);
        keywords.insert("or", Or);
        keywords.insert("print", Print);
        keywords.insert("return", Return);
        keywords.insert("super", Super);
        keywords.insert("this", This);
        keywords.insert("true", True);
        keywords.insert("var", Var);
        keywords.insert("while", While);

        Scanner {
            source: source.chars().peekable(),
            line: 1,
            start: 0,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> ScannerResult<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            self.skip_whitespace();
            match self.source.next() {
                Some(c) => {
                    if !self.skip_comments(c) {
                        match self.scan_token(c) {
                            Ok(token) => tokens.push(token),
                            Err(err) => return Err(err),
                        }
                    }
                }
                None => {
                    tokens.push(self.make_token(TokenType::EOF));
                    break;
                }
            }
        }

        Ok(tokens)
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, None, None, self.line)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.source.peek() {
            if !c.is_whitespace() {
                return;
            } else if c == '\n' {
                self.line += 1;
            }

            self.source.next();
        }
    }

    fn skip_comments(&mut self, c: char) -> bool {
        if c == '/' && self.source.peek() == Some(&'/') {
            while let Some(&c) = self.source.peek() {
                self.source.next();
                if c == '\n' {
                    self.line += 1;
                    return true;
                }
            }
        }
        false
    }

    fn scan_token(&mut self, c: char) -> ScannerResult<Token> {
        use crate::token::TokenType::*;

        match c {
            '(' => Ok(self.make_token(LeftParen)),
            ')' => Ok(self.make_token(RightParen)),
            '{' => Ok(self.make_token(LeftBrace)),
            '}' => Ok(self.make_token(RightBrace)),
            ',' => Ok(self.make_token(Comma)),
            '.' => Ok(self.make_token(Dot)),
            '-' => Ok(self.make_token(Minus)),
            '+' => Ok(self.make_token(Plus)),
            ';' => Ok(self.make_token(SemiColon)),
            '*' => Ok(self.make_token(Star)),

            // Could either be comment or slash
            '/' => Ok(self.make_token(Slash)),

            // Need to peek ahead to check for next char
            '=' => Ok(self.scan_operator(Equal, EqualEqual)),
            '!' => Ok(self.scan_operator(Bang, BangEqual)),
            '<' => Ok(self.scan_operator(Less, LessEqual)),
            '>' => Ok(self.scan_operator(Greater, GreaterEqual)),

            '"' => self.scan_string(),

            c @ _ => {
                if c.is_digit(10) {
                    self.scan_number(c)
                } else if c.is_alphabetic() || c == '_' {
                    self.scan_identifier(c)
                } else {
                    Err(ScannerError::UnknownCharacter(c, self.line))
                }
            }
        }
    }

    fn scan_operator(&mut self, inequality_type: TokenType, equality_type: TokenType) -> Token {
        if self.source.peek() == Some(&'=') {
            self.source.next();
            self.make_token(equality_type)
        } else {
            self.make_token(inequality_type)
        }
    }

    fn scan_string(&mut self) -> ScannerResult<Token> {
        let mut captured_string = String::new();
        let start_line = self.line;
        while let Some(&c) = self.source.peek() {
            if c == '"' {
                self.source.next();
                return Ok(Token::new(
                    TokenType::String,
                    Some(captured_string.clone()),
                    Some(Literal::Str(captured_string)),
                    self.line,
                ));
            } else if c == '\n' {
                self.line += 1;
            }
            captured_string.push(self.source.next().unwrap());
        }

        Err(ScannerError::UntermiantedString(start_line))
    }

    fn scan_number(&mut self, c: char) -> ScannerResult<Token> {
        let mut captured_number = String::new();
        captured_number.push(c);

        while let Some(&c) = self.source.peek() {
            if c == '.' {
                if captured_number.contains(".") {
                    return Err(ScannerError::InvalidCharacter('.', self.line));
                } else {
                    captured_number.push(c)
                }
            } else if c.is_digit(10) {
                captured_number.push(c);
            } else {
                break;
            }
            self.source.next();
        }

        if let Ok(parsed_number) = captured_number.parse::<f64>() {
            return Ok(Token::new(
                TokenType::Number,
                Some(captured_number),
                Some(Literal::Float(parsed_number)),
                self.line,
            ));
        } else {
            Err(ScannerError::InvalidTerm(captured_number, self.line))
        }
    }

    fn scan_identifier(&mut self, c: char) -> ScannerResult<Token> {
        let mut captured_identifier = String::new();
        captured_identifier.push(c);

        while let Some(&c) = self.source.peek() {
            if !c.is_alphabetic() && c != '_' && !c.is_digit(10) {
                break;
            }
            captured_identifier.push(c);
            self.source.next();
        }

        match self.keywords.get(&*captured_identifier) {
            Some(keyword_token) => Ok(self.make_token(keyword_token.clone())),
            None => Ok(Token::new(
                TokenType::Identifier,
                Some(captured_identifier),
                None,
                self.line,
            )),
        }
    }
}
