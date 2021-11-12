use std::{iter::Peekable, str::Chars};

use crate::token::{Token, TokenType};

pub struct Scanner<'a> {
    source: Peekable<Chars<'a>>,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source: source.chars().peekable(),
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        if let Some(c) = self.source.next() {
            return match c {
                '(' => self.make_token(TokenType::LeftParen, Some("(")),
                ')' => self.make_token(TokenType::RightParen, Some(")")),
                '{' => self.make_token(TokenType::LeftBrace, Some("{")),
                '}' => self.make_token(TokenType::RightBrace, Some("}")),
                ';' => self.make_token(TokenType::Semicolon, Some(";")),
                ',' => self.make_token(TokenType::Comma, Some(",")),
                '.' => self.make_token(TokenType::Dot, Some(".")),
                '-' => self.make_token(TokenType::Minus, Some("-")),
                '+' => self.make_token(TokenType::Plus, Some("+")),
                '/' => self.make_token(TokenType::Slash, Some("/")),
                '*' => self.make_token(TokenType::Star, Some("*")),

                // Peek ahead 1 char
                '!' => self.match_binary(TokenType::Bang, TokenType::BangEqual, '!'),
                '=' => self.match_binary(TokenType::Equal, TokenType::EqualEqual, '='),
                '<' => self.match_binary(TokenType::Less, TokenType::LessEqual, '<'),
                '>' => self.match_binary(TokenType::Greater, TokenType::GreaterEqual, '>'),

                '"' => self.match_string(),
                '0'..='9' => self.match_digit(c),
                'a'..='z' | 'A'..='Z' | '_' => self.match_identifier(c),
                '\n' => self.make_token(TokenType::Eof, None),
                _ => panic!("Invalid token: {}, line {}", c, self.line),
            };
        } else {
            panic!("Invalid token, line {}", self.line)
        }
    }

    fn match_binary(
        &mut self,
        inequality_type: TokenType,
        equality_type: TokenType,
        curr_char: char,
    ) -> Token {
        let lexeme = curr_char.to_string();
        if let Some('=') = self.source.peek() {
            self.source.next();
            self.make_token(equality_type, Some((lexeme + "=").as_str()))
        } else {
            self.make_token(inequality_type, Some(lexeme.as_str()))
        }
    }

    fn match_string(&mut self) -> Token {
        let mut captured_string = String::new();
        let start_line = self.line;
        while let Some(&c) = self.source.peek() {
            if c == '"' {
                self.source.next();
                // TODO: check if token should hold literal value
                return self.make_token(TokenType::String, Some(captured_string.as_str()));
            } else if c == '\n' {
                self.line += 1;
            }
            captured_string.push(self.source.next().unwrap());
        }
        panic!("Unterminated string: line {}", start_line);
    }

    fn match_digit(&mut self, captured_digit: char) -> Token {
        let mut captured_digit = captured_digit.to_string();
        while self.is_digit() {
            let c = self.source.next().unwrap();
            captured_digit.push(c);
        }

        if let Some(&'.') = self.source.peek() {
            captured_digit.push(self.source.next().unwrap());
            if self.is_digit() {
                while self.is_digit() {
                    let c = self.source.next().unwrap();
                    captured_digit.push(c);
                }
            } else {
                panic!("Expected number after decimal point: line {}", self.line);
            }
        }

        // TODO: numbers need literal
        self.make_token(TokenType::Number, Some(captured_digit.as_str()))
    }

    fn match_identifier(&mut self, captured_name: char) -> Token {
        let mut identifier = captured_name.to_string();
        while let Some(&c) = self.source.peek() {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.source.next();
            } else {
                break;
            }
        }

        // check if its a keyword
        match identifier.chars().next().unwrap() {
            'a' => self.check_keyword("and", TokenType::And, identifier),
            'c' => self.check_keyword("class", TokenType::Class, identifier),
            'e' => self.check_keyword("else", TokenType::Else, identifier),
            'f' => {
                if let Some(c) = identifier.chars().nth(1) {
                    match c {
                        'a' => return self.check_keyword("false", TokenType::False, identifier),
                        'o' => return self.check_keyword("for", TokenType::For, identifier),
                        'u' => return self.check_keyword("fun", TokenType::Fun, identifier),
                        _ => {}
                    };
                }
                self.make_token(TokenType::Identifier, Some(identifier.as_str()))
            }
            'i' => self.check_keyword("if", TokenType::If, identifier),
            'n' => self.check_keyword("nil", TokenType::Nil, identifier),
            'o' => self.check_keyword("or", TokenType::Or, identifier),
            'p' => self.check_keyword("print", TokenType::Print, identifier),
            'r' => self.check_keyword("return", TokenType::Return, identifier),
            's' => self.check_keyword("super", TokenType::Super, identifier),
            't' => {
                if let Some(c) = identifier.chars().nth(1) {
                    match c {
                        'h' => return self.check_keyword("this", TokenType::This, identifier),
                        'r' => return self.check_keyword("true", TokenType::True, identifier),
                        _ => {}
                    }
                }
                self.make_token(TokenType::Identifier, Some(identifier.as_str()))
            }
            'v' => self.check_keyword("var", TokenType::Var, identifier),
            'w' => self.check_keyword("while", TokenType::While, identifier),
            _ => self.make_token(TokenType::Identifier, Some(identifier.as_str())),
        }
    }

    fn check_keyword(
        &self,
        rest: &str,
        token_type: TokenType,
        captured_identifier: String,
    ) -> Token {
        if captured_identifier.eq(rest) {
            return self.make_token(token_type, Some(captured_identifier.as_str()));
        }
        self.make_token(TokenType::Identifier, Some(captured_identifier.as_str()))
    }

    fn is_digit(&mut self) -> bool {
        if let Some(c) = self.source.peek() {
            return ('0'..='9').contains(c);
        }
        false
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.source.peek() {
            if !c.is_whitespace() {
                return;
            } else if c == '\n' {
                self.line += 1;
                break;
            } else if c == '/' {
                self.skip_comments();
                break;
            }
            self.source.next();
        }
    }

    fn skip_comments(&mut self) {
        if let Some(&'/') = self.source.peek() {
            while let Some(&c) = self.source.peek() {
                self.source.next();
                if c == '\n' {
                    self.line += 1;
                    break;
                }
            }
        }
    }

    fn make_token(&self, token_type: TokenType, lexeme: Option<&str>) -> Token {
        Token::new(token_type, lexeme.map(|c| c.to_string()), self.line)
    }
}
