pub struct Lexer {
    input: String,
    position: usize,
    total: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Identifiers and literals
    NIDENT,
    IDENT,

    // bounds
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // types
    INT,

    // assignments & comparisons
    LT,
    GT,
    EQ,

    // Delimiters
    COMMA,
    SEMICOLON,

    // Keywords
    RETURN,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let sz = input.len();
        Lexer {
            input,
            position: 0,
            total: sz,
        }
    }

    pub fn next_token(&mut self) -> Option<char> {
        if self.position >= self.total {
            None
        } else {
            let ch = self.input.chars().nth(self.position);
            self.position += 1;
            ch
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        loop {
            let ch = self.next_token();
            if ch.is_none() {
                break;
            }
            let ch = ch.unwrap();
            if should_break_running_token(&ch, &mut current_token) {
                let token = identifier_mapper(&current_token);
                tokens.push(token);
                current_token = String::new();
            } else {
                if ch.is_whitespace() {
                    continue;
                }

                let special_token = special_mapper(&ch);
                if let Some(t) = special_token {
                    tokens.push(t);
                } else {
                    current_token.push(ch);
                }
                continue;
            }

            if ch.is_whitespace() {
                continue;
            }

            let special_token = special_mapper(&ch);
            if let Some(t) = special_token {
                tokens.push(t);
            }
        }
        if !current_token.is_empty() {
            let token = identifier_mapper(&current_token);
            tokens.push(token);
        }
        tokens
    }
}

fn literal_mapper(literal: &str) -> Token {
    // check if all are digits
    if literal.chars().all(char::is_numeric) {
        return Token {
            token_type: TokenType::NIDENT,
            literal: literal.to_string(),
        };
    }
    Token {
        token_type: TokenType::IDENT,
        literal: literal.to_string(),
    }
}
fn identifier_mapper(ident: &str) -> Token {
    match ident {
        "return" => Token {
            token_type: TokenType::RETURN,
            literal: ident.to_string(),
        },
        "int" => Token {
            token_type: TokenType::INT,
            literal: ident.to_string(),
        },
        _ => literal_mapper(ident),
    }
}

fn special_mapper(ch: &char) -> Option<Token> {
    match ch {
        '(' => Some(Token {
            token_type: TokenType::LPAREN,
            literal: ch.to_string(),
        }),
        ')' => Some(Token {
            token_type: TokenType::RPAREN,
            literal: ch.to_string(),
        }),
        '{' => Some(Token {
            token_type: TokenType::LBRACE,
            literal: ch.to_string(),
        }),
        '}' => Some(Token {
            token_type: TokenType::RBRACE,
            literal: ch.to_string(),
        }),
        ',' => Some(Token {
            token_type: TokenType::COMMA,
            literal: ch.to_string(),
        }),
        ';' => Some(Token {
            token_type: TokenType::SEMICOLON,
            literal: ch.to_string(),
        }),
        '=' => Some(Token {
            token_type: TokenType::EQ,
            literal: ch.to_string(),
        }),
        _ => None,
    }
}

fn should_break_running_token(ch: &char, curr: &mut String) -> bool {
    if curr.is_empty() {
        return false;
    }
    if ch.is_whitespace()
        || ch == &','
        || ch == &'('
        || ch == &')'
        || ch == &'{'
        || ch == &'}'
        || ch == &';'
        || ch == &'='
    {
        return true;
    }
    false
}
