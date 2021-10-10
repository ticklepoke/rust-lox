pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {}
