use corrozy_core::parser::{ast::{AstNode, Expression}, parser::CorrozyParserImpl};

fn parse_single_statement(code: &str) -> AstNode {
    let mut parser: CorrozyParserImpl = CorrozyParserImpl::new();
    let ast = parser.parse(code).unwrap();
    assert_eq!(ast.len(), 1);
    ast.into_iter().next().unwrap()
}

fn extract_literal_value(expr: &Expression) -> String {
    match expr {
        Expression::Literal(lit) => lit.to_php(),
        _ => panic!("Expected literal"),
    }
}

#[test]
fn parser() {
    let ast = parse_single_statement("let x: int = 1;");
    
    match ast {
        AstNode::VariableDeclaration {
            name,
            var_type,
            value,
        } => {
            assert_eq!(name, "x");
            assert_eq!(var_type.as_ref().unwrap(), "int");
            assert_eq!(extract_literal_value(&value), "1");
        },
        other => panic!("Unexpected AST node: {:?}", other),
    }
}