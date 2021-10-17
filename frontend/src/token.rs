use crate::literal::Literal;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or Two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Misc
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(lexeme) = &self.lexeme {
            write!(f, "{:?} {} {:?}", self.token_type, lexeme, self.literal)
        } else {
            write!(f, "{:?} '' {:?}", self.token_type, self.literal)
        }
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: Option<String>,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
