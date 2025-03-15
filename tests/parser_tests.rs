use crusty::lexer::{Lexer, Token};
use crusty::parser::{Node, NodeType, Parser, AST};

#[test]
fn test_parse_full_program() {
    let input = "int main() { return 2; }";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let expected_ast = AST::new(Node {
        node_type: NodeType::Program,
        value: "int id() { return 2; }".to_string(),
        children: vec![Node {
            node_type: NodeType::Function,
            value: "int id() { return 2; }".to_string(),
            children: vec![Node {
                node_type: NodeType::Statement,
                value: "return 2;".to_string(),
                children: vec![Node {
                    node_type: NodeType::Expression,
                    value: "2".to_string(),
                    children: vec![],
                }],
            }],
        }],
    });

    assert_eq!(ast.unwrap(), expected_ast);
}

#[test]
fn test_parse_full_program_big_number() {
    let input = "int main() { return 23192371 ; }";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let expected_ast = AST::new(Node {
        node_type: NodeType::Program,
        value: "int id() { return 23192371; }".to_string(),
        children: vec![Node {
            node_type: NodeType::Function,
            value: "int id() { return 23192371; }".to_string(),
            children: vec![Node {
                node_type: NodeType::Statement,
                value: "return 23192371;".to_string(),
                children: vec![Node {
                    node_type: NodeType::Expression,
                    value: "23192371".to_string(),
                    children: vec![],
                }],
            }],
        }],
    });

    assert_eq!(ast.unwrap(), expected_ast);
}
