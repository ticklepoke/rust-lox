use std::fmt;

pub enum Error {
    ScannerError(ScannerError),
    InterpreterError(InterpreterError),
    ParserError(ParserError),
}

#[derive(Debug)]
pub enum ScannerError {
    UnknownCharacter(char, usize),
    UntermiantedString(usize),
    InvalidCharacter(char, usize),
    InvalidTerm(String, usize),
    UnknownError,
}

#[derive(Debug)]
pub enum InterpreterError {
    InvalidCoercion(String),
    InvalidAstType,
    UndefinedVariable(String),
    MismatchFunctionArity,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(String, usize),
    GenericError(String, usize),
    InvalidAssignmentTarget,
    ArgumentCountExceeded,
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
                write!(f, "Invalid term {} at line {}", s.as_str(), line_number)
            }
            ScannerError::UnknownError => write!(f, "Unknown error"),
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
            ScannerError::UnknownError => 0,
        }
    }
}
