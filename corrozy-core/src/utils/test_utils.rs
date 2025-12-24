use anyhow::Result;

use crate::{Config, config::{NamespaceConfig, NamespaceMode, TranspilerConfig}, language::{AstNode, features::expression::ast::Expression, parser::CorrozyParserImpl}};

pub fn default_corrozy_config() -> Config {
    Config {
        transpiler: TranspilerConfig {
            src_dir: "src".to_string(),
            output_dir: "out".to_string(),
            strict_types: true,
            include_comments: false,
        },
        namespace: NamespaceConfig {
            base_namespace: "MyApp".to_string(),
            separator: "\\".to_string(),
            mode: NamespaceMode::Auto,
        },
    }
}

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