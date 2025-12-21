use anyhow::Result;
use corrozy_core::language::{AstNode, features::expression::ast::Expression, parser::CorrozyParserImpl};

pub fn parse_single_statement(code: &str) -> AstNode {
    let mut parser: CorrozyParserImpl = CorrozyParserImpl::new();
    let ast = parser.parse(code).unwrap();
    assert_eq!(ast.len(), 1, "Expected exactly one statement");
    ast.into_iter().next().unwrap()
}

pub fn parse_with_error(code: &str) -> Result<Vec<AstNode>> {
    let mut parser: CorrozyParserImpl = CorrozyParserImpl::new();
    parser.parse(code)
}

pub fn extract_literal_value(expr: &Expression) -> String {
    match expr {
        Expression::Literal(lit) => lit.to_php(),
        _ => panic!("Expected literal, got: {:?}", expr),
    }
}