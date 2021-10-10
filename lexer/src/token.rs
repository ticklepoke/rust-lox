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

#[derive(Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    lexeme: Option<String>,
    literal: Option<Literal>,
    line: usize,
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

    pub fn to_string(&self) -> String {
        if let Some(lexeme) = &self.lexeme {
            format!("{:?} {} {:?}", self.token_type, lexeme, self.literal)
        } else {
            format!("{:?} '' {:?}", self.token_type, self.literal)
        }
    }
}
