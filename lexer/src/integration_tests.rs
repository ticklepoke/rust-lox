use crate::scanner;
use crate::token::{Token, TokenType};

#[test]
fn test_scanner() {
    let mut scanner = scanner::Scanner::new("and");
    if let Ok(tokens) = scanner.scan_tokens() {
        let mut expected_tokens = Vec::new();
        expected_tokens.push(Token::new(TokenType::And, None, None, 1));
        expected_tokens.push(Token::new(TokenType::EOF, None, None, 1));
        assert_eq!(tokens, expected_tokens);
    }
}
