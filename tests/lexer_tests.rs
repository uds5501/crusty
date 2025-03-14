use crusty::lexer::{Lexer, Token, TokenType};

#[test]
fn test_single_characters() {
    let input = "(){};,=";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.lex();
    let expected_tokens = vec![
        Token { token_type: TokenType::LPAREN, literal: "(".to_string() },
        Token { token_type: TokenType::RPAREN, literal: ")".to_string() },
        Token { token_type: TokenType::LBRACE, literal: "{".to_string() },
        Token { token_type: TokenType::RBRACE, literal: "}".to_string() },
        Token { token_type: TokenType::SEMICOLON, literal: ";".to_string() },
        Token { token_type: TokenType::COMMA, literal: ",".to_string() },
        Token { token_type: TokenType::EQ, literal: "=".to_string() },
    ];
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_keywords_and_identifiers() {
    let input = "return int myVar";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.lex();
    let expected_tokens = vec![
        Token { token_type: TokenType::RETURN, literal: "return".to_string() },
        Token { token_type: TokenType::INT, literal: "int".to_string() },
        Token { token_type: TokenType::IDENT, literal: "myVar".to_string() },
    ];
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_integer_literals() {
    let input = "123 456";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.lex();
    let expected_tokens = vec![
        Token { token_type: TokenType::NIDENT, literal: "123".to_string() },
        Token { token_type: TokenType::NIDENT, literal: "456".to_string() },
    ];
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_full_program() {
    let input = "int main() { return 2; }";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.lex();
    let expected_tokens = vec![
        Token { token_type: TokenType::INT, literal: "int".to_string() },
        Token { token_type: TokenType::IDENT, literal: "main".to_string() },
        Token { token_type: TokenType::LPAREN, literal: "(".to_string() },
        Token { token_type: TokenType::RPAREN, literal: ")".to_string() },
        Token { token_type: TokenType::LBRACE, literal: "{".to_string() },
        Token { token_type: TokenType::RETURN, literal: "return".to_string() },
        Token { token_type: TokenType::NIDENT, literal: "2".to_string() },
        Token { token_type: TokenType::SEMICOLON, literal: ";".to_string() },
        Token { token_type: TokenType::RBRACE, literal: "}".to_string() },
    ];
    assert_eq!(tokens, expected_tokens);
}
