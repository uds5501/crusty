use crate::lexer::{Token, TokenType};
// grammar

// P -> F
// F -> int id ( ) { S }
// S -> return E ;
// E -> int_literal

#[derive(Debug, PartialEq, Clone)]
pub enum NodeType {
    Program,
    Function,
    Statement,
    Expression,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
    pub value: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AST {
    root: Node,
}

impl AST {
    pub fn new(root: Node) -> Self {
        AST { root }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Option<Token>,
    idx: usize,
    tot: usize,
    errors: Vec<String>,
}

impl Parser {
    fn next_token(&mut self) {
        if self.idx >= self.tokens.len() {
            self.current_token = None;
        } else {
            self.current_token = Some(self.tokens[self.idx].clone());
            self.idx += 1;
        }
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        let x = tokens.len();
        let mut parser = Parser {
            tokens,
            current_token: None,
            idx: 0,
            tot: x,
            errors: Vec::new(),
        };
        parser.next_token();
        parser
    }

    // P -> F
    pub fn parse(&mut self) -> Result<AST, String> {
        if self.current_token.is_none() {
            return Err("No tokens to parse".to_string());
        }
        if !self.is_expected_type(TokenType::INT) {
            return Err("Expected int".to_string());
        }
        let function = self.parse_function();
        if function.is_err() {
            return function;
        }

        Ok(AST {
            root: Node {
                node_type: NodeType::Program,
                value: function.clone()?.root.value,
                children: vec![function?.root],
            },
        })
    }

    // F -> int id ( ) { S }
    fn parse_function(&mut self) -> Result<AST, String> {
        let mut valid = true;
        let mut end = false;
        // use this as a single parser for single grammar.
        while self.current_token.is_some() && valid {
            if !self.is_expected_type(TokenType::INT) {
                valid = false;
            } else {
                self.next_token();
            }

            if self.is_expected_type(TokenType::IDENT) {
                self.next_token();
            } else {
                valid = false;
            }
            if self.is_expected_type(TokenType::LPAREN) {
                self.next_token();
            } else {
                valid = false;
            }
            if self.is_expected_type(TokenType::RPAREN) {
                self.next_token();
            } else {
                valid = false;
            }
            if self.is_expected_type(TokenType::LBRACE) {
                self.next_token();
            } else {
                valid = false;
            }
            // check for errors later.
            let statement = self.parse_statement();
            if statement.is_err() {
                return statement;
            }

            if self.is_expected_type(TokenType::RBRACE) {
                self.next_token();
            } else {
                valid = false;
            }
            end = true;
            if !valid {
                self.errors.push(format!(
                    "Expected ident, found {:?}",
                    self.current_token.as_ref().unwrap()
                ));
                break;
            }
            return if end {
                Ok({
                    AST {
                        root: Node {
                            node_type: NodeType::Function,
                            value: format!("int id() {{ {} }}", statement.clone()?.root.value),
                            children: vec![statement?.root],
                        },
                    }
                })
            } else {
                Err("Expected proper function;".to_string())
            };
        }
        Err(self.errors.first().unwrap().to_string())
    }

    // S -> return E ;
    fn parse_statement(&mut self) -> Result<AST, String> {
        let mut valid = true;
        while self.current_token.is_some() && valid {
            if !self.is_expected_type(TokenType::RETURN) {
                valid = false;
            }
            self.next_token();

            let expression = self.parse_expression();
            if expression.is_err() {
                return expression;
            }
            if !self.is_expected_type(TokenType::SEMICOLON) {
                valid = false;
            }
            self.next_token();
            if !valid {
                self.errors.push("Expected proper statement;".to_string());
                break;
            }
            return Ok({
                AST {
                    root: Node {
                        node_type: NodeType::Statement,
                        value: format!("return {};", expression.clone()?.root.value),
                        children: vec![expression?.root],
                    },
                }
            });
        }
        Err(self.errors.first().unwrap().to_string())
    }

    // E -> int_literal
    fn parse_expression(&mut self) -> Result<AST, String> {
        let value: String;
        if !self.is_expected_type(TokenType::NIDENT) {
            return Err("Expected int literal".to_string());
        }
        if let Some(token) = &self.current_token {
            value = token.literal.clone();
            self.next_token();
            Ok(AST {
                root: Node {
                    node_type: NodeType::Expression,
                    children: vec![],
                    value,
                },
            })
        } else {
            Err("Expected int literal".to_string())
        }
    }

    fn is_expected_type(&self, token_type: TokenType) -> bool {
        if let Some(token) = &self.current_token {
            return token.token_type == token_type;
        }
        false
    }
}
