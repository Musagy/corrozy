use corrozy_core::parser::ast::{AstNode};

use crate::utils::test_utils::{extract_literal_value, parse_single_statement, parse_with_error};

#[test]
fn test_variable_declaration_with_type() {
    let ast = parse_single_statement("let x: int = 1;");
    
    match ast {
        AstNode::VariableDeclaration { name, var_type, value } => {
            assert_eq!(name, "x");
            assert_eq!(var_type.as_ref().unwrap(), "int");
            assert_eq!(extract_literal_value(&value), "1");
        },
        other => panic!("Expected VariableDeclaration, got: {:?}", other),
    }
}

#[test]
fn test_variable_declaration_without_type() {
    let ast = parse_single_statement("let y = \"hello\";");
    
    match ast {
        AstNode::VariableDeclaration { name, var_type, value } => {
            assert_eq!(name, "y");
            assert!(var_type.is_none());
            assert_eq!(extract_literal_value(&value), "\"hello\"");
        },
        other => panic!("Expected VariableDeclaration, got: {:?}", other),
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
        other => panic!("Expected ConstantDeclaration, got: {:?}", other),
    }
}

#[test]
fn test_constant_declaration_without_type() {
    let ast = parse_single_statement("const MAX = 100;");
    
    match ast {
        AstNode::ConstantDeclaration { name, const_type, value } => {
            assert_eq!(name, "MAX");
            assert!(const_type.is_none());
            assert_eq!(extract_literal_value(&value), "100");
        },
        other => panic!("Expected ConstantDeclaration, got: {:?}", other),
    }
}

#[test] 
fn test_invalid_variable_syntax() {
    let result = parse_with_error("let = 5;"); // Sin nombre
    assert!(result.is_err());
    
    let result = parse_with_error("let x 5;"); // Sin =
    assert!(result.is_err());
    
    let result = parse_with_error("let x = ;"); // Sin valor
    assert!(result.is_err());
}

#[test]
fn test_invalid_constant_syntax() {
    let result = parse_with_error("const = 5;"); // Sin nombre
    assert!(result.is_err());
    
    let result = parse_with_error("const X;"); // Sin valor
    assert!(result.is_err());
}