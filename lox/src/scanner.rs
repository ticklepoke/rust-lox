use std::fmt;

use crate::token::Token;

type ScannerResult<T> = Result<T, ScannerError>;

#[derive(Debug)]
pub enum ScannerError {
    UnknownCharacter(char, usize),
    UntermiantedString(usize),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScannerError::UnknownCharacter(c, line_number) => {
                write!(f, "Unrecognised character {} at line {}", c, line_number)
            }
            ScannerError::UntermiantedString(line_number) => {
                write!(f, "Unterminated string at line {}", line_number)
            }
        }
    }
}

impl ScannerError {
    pub fn line(&self) -> usize {
        match *self {
            ScannerError::UnknownCharacter(_, line_number) => line_number,
            ScannerError::UntermiantedString(line_number) => line_number,
        }
    }
}

pub struct Scanner {
    source: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> ScannerResult<Vec<Token>> {
        let tokens: Vec<Token> = Vec::new();

        Ok(tokens)
    }
}
