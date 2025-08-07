use corrozy_core::parser::ast::{AstNode, Expression};

fn extract_literal_value(expr: &Expression) -> String {
    match expr {
        Expression::Literal(lit) => lit.to_php(),
        _ => panic!("Expected literal"),
    }
}

#[cfg(test)]
mod variable_tests {
    use crate::utils::test_utils::parse_single_statement;

    use super::*;

    #[test]
    fn test_variable_declaration_with_type() {
        let ast = parse_single_statement("let x: int = 1;");
        
        match ast {
            AstNode::VariableDeclaration { name, var_type, value } => {
                assert_eq!(name, "x");
                assert_eq!(var_type.as_ref().unwrap(), "int");
                assert_eq!(extract_literal_value(&value), "1");
            },
            other => panic!("Unexpected AST node: {:?}", other),
        }
    }

    #[test]
    fn test_variable_declaration_without_type() {
        let ast = parse_single_statement("let nombre = \"Diego\";");
        
        match ast {
            AstNode::VariableDeclaration { name, var_type, value } => {
                assert_eq!(name, "nombre");
                assert!(var_type.is_none());
                assert_eq!(extract_literal_value(&value), "\"Diego\"");
            },
            other => panic!("Unexpected AST node: {:?}", other),
        }
    }

    #[test]
    fn test_constant_declaration_with_type() {
        let ast = parse_single_statement("const PI: float = 3.14;");
        
        match ast {
            AstNode::ConstantDeclaration { name, const_type, value } => {
                assert_eq!(name, "PI");
                assert_eq!(const_type.as_ref().unwrap(), "float");
                assert_eq!(extract_literal_value(&value), "3.14");
            },
            other => panic!("Unexpected AST node: {:?}", other),
        }
    }

    #[test]  
    fn test_constant_declaration_without_type() {
        let ast = parse_single_statement("const MAX_SIZE = 100;");
        
        match ast {
            AstNode::ConstantDeclaration { name, const_type, value } => {
                assert_eq!(name, "MAX_SIZE");
                assert!(const_type.is_none());
                assert_eq!(extract_literal_value(&value), "100");
            },
            other => panic!("Unexpected AST node: {:?}", other),
        }
    }
}

#[cfg(test)]
mod error_tests {
    use crate::utils::test_utils::parse_single_statement;

    #[test]
    #[should_panic]
    fn test_missing_semicolon() {
        parse_single_statement("let x = 1"); // Sin ;
    }

    #[test]
    #[should_panic] 
    fn test_missing_assignment() {
        parse_single_statement("let x;"); // Sin = valor
    }

    #[test]
    #[should_panic]
    fn test_invalid_type() {
        parse_single_statement("let x: invalidtype = 1;");
    }
}