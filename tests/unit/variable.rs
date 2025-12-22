use corrozy_core::language::{AstNode, };
use crate::utils::test_utils::{extract_literal_value, parse_single_statement, parse_with_error};

#[cfg(test)]
mod variable_declaration_tests {
    use super::*;

    #[test]
    /// Test variable declaration with explicit type annotation
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
    /// Test variable declaration with double quoted string
    fn test_variable_declaration_double_quotes() {
        let ast = parse_single_statement("let nombre = \"Diego\";");
        
        match ast {
            AstNode::VariableDeclaration { name, var_type, value } => {
                assert_eq!(name, "nombre");
                assert!(var_type.is_none());
                assert_eq!(extract_literal_value(&value), "\"Diego\"");
            },
            other => panic!("Expected VariableDeclaration, got: {:?}", other),
        }
    }

    #[test]
    /// Test variable declaration with single quoted string
    fn test_variable_declaration_single_quotes() {
        let ast = parse_single_statement("let nombre = 'Diego';");
        
        match ast {
            AstNode::VariableDeclaration { name, var_type, value } => {
                assert_eq!(name, "nombre");
                assert!(var_type.is_none());
                assert_eq!(extract_literal_value(&value), "'Diego'");
            },
            other => panic!("Expected VariableDeclaration, got: {:?}", other),
        }
    }

    #[test]
    /// Test variable declaration with special characters in name
    fn test_variable_declaration_special_names() {
        let ast = parse_single_statement("let _variable_name = 42;");
        
        match ast {
            AstNode::VariableDeclaration { name, var_type, value } => {
                assert_eq!(name, "_variable_name");
                assert!(var_type.is_none());
                assert_eq!(extract_literal_value(&value), "42");
            },
            other => panic!("Expected VariableDeclaration, got: {:?}", other),
        }
    }

    #[test]
    /// Test variable declaration with empty string
    fn test_variable_declaration_empty_string() {
        let ast = parse_single_statement("let empty = \"\";");
        
        match ast {
            AstNode::VariableDeclaration { name, var_type, value } => {
                assert_eq!(name, "empty");
                assert!(var_type.is_none());
                assert_eq!(extract_literal_value(&value), "\"\"");
            },
            other => panic!("Expected VariableDeclaration, got: {:?}", other),
        }
    }
}

#[cfg(test)]
mod constant_declaration_tests {
    use super::*;

    #[test]
    /// Test constant declaration with explicit type annotation
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
    /// Test constant declaration with type inference
    fn test_constant_declaration_without_type() {
        let ast = parse_single_statement("const MAX_SIZE = 100;");
        
        match ast {
            AstNode::ConstantDeclaration { name, const_type, value } => {
                assert_eq!(name, "MAX_SIZE");
                assert!(const_type.is_none());
                assert_eq!(extract_literal_value(&value), "100");
            },
            other => panic!("Expected ConstantDeclaration, got: {:?}", other),
        }
    }

    #[test]
    /// Test constant declaration with large numbers
    fn test_constant_declaration_large_numbers() {
        let ast = parse_single_statement("const BIG_NUMBER = 9999999999;");
        
        match ast {
            AstNode::ConstantDeclaration { name, const_type, value } => {
                assert_eq!(name, "BIG_NUMBER");
                assert!(const_type.is_none());
                assert_eq!(extract_literal_value(&value), "9999999999");
            },
            other => panic!("Expected ConstantDeclaration, got: {:?}", other),
        }
    }
}

#[cfg(test)]
mod literal_generation_tests {
    use corrozy_core::language::features::expression::ast::{Literal, StringType};

    #[test]
    /// Test integer literal PHP generation
    fn test_integer_literal_generation() {
        let literal = Literal::Integer(42);
        assert_eq!(literal.to_php(), "42");
    }

    #[test]
    /// Test float literal PHP generation
    fn test_float_literal_generation() {
        let literal = Literal::Float(3.14);
        assert_eq!(literal.to_php(), "3.14");
    }

    #[test]
    /// Test boolean literal PHP generation
    fn test_boolean_literal_generation() {
        let true_literal = Literal::Boolean(true);
        let false_literal = Literal::Boolean(false);
        
        assert_eq!(true_literal.to_php(), "true");
        assert_eq!(false_literal.to_php(), "false");
    }

    #[test]
    /// Test interpolated string generation with comprehensive cases
    fn test_interpolated_string_generation() {
        // Basic interpolated string
        let literal = Literal::String(StringType::Interpolated("Hello world".to_string()));
        assert_eq!(literal.to_php(), "\"Hello world\"");
        
        // With PHP variables
        let literal_vars = Literal::String(StringType::Interpolated("Hello $name, price: $price".to_string()));
        assert_eq!(literal_vars.to_php(), "\"Hello $name, price: $price\"");
        
        // Empty string
        let empty_literal = Literal::String(StringType::Interpolated("".to_string()));
        assert_eq!(empty_literal.to_php(), "\"\"");
    }

    #[test]
    /// Test raw string generation with comprehensive cases
    fn test_raw_string_generation() {
        // Basic raw string
        let literal = Literal::String(StringType::Raw("Hello world".to_string()));
        assert_eq!(literal.to_php(), "'Hello world'");
        
        // Raw string with PHP variables (no interpolation)
        let literal_vars = Literal::String(StringType::Raw("Hello $name".to_string()));
        assert_eq!(literal_vars.to_php(), "'Hello $name'");
        
        // Empty raw string
        let empty_literal = Literal::String(StringType::Raw("".to_string()));
        assert_eq!(empty_literal.to_php(), "''");
    }

    #[test]
    /// Test strings with escape characters
    fn test_string_with_escapes() {
        let literal = Literal::String(StringType::Interpolated("Hello\\nWorld\\t".to_string()));
        assert_eq!(literal.to_php(), "\"Hello\\nWorld\\t\"");
        
        let raw_literal = Literal::String(StringType::Raw("Path: C:\\\\Users\\\\Name".to_string()));
        assert_eq!(raw_literal.to_php(), "'Path: C:\\\\Users\\\\Name'");
    }

    #[test]
    /// Test extreme numeric values
    fn test_extreme_numeric_literals() {
        let large_int = Literal::Integer(i64::MAX);
        assert_eq!(large_int.to_php(), &*i64::MAX.to_string());
        
        let small_float = Literal::Float(0.00001);
        assert_eq!(small_float.to_php(), "0.00001");
    }
}

#[cfg(test)]
mod syntax_error_tests {
    use super::*;

    #[test]
    /// Test that missing semicolon causes parser error
    fn test_missing_semicolon() {
        let result = parse_with_error("let x = 1");
        assert!(result.is_err());
    }

    #[test]
    /// Test that missing assignment in variable declaration causes parser error
    fn test_missing_variable_assignment() {
        let result = parse_with_error("let x;");
        assert!(result.is_err());
    }

    #[test]
    /// Test that invalid type annotation causes parser error
    fn test_invalid_type() {
        let result = parse_with_error("let x: invalide = 1;");
        assert!(result.is_err());
    }

    #[test]
    /// Test various invalid variable declaration syntax patterns
    fn test_invalid_variable_syntax() {
        let result = parse_with_error("let = 5;");
        assert!(result.is_err());
        
        let result = parse_with_error("let x 5;");
        assert!(result.is_err());
        
        let result = parse_with_error("let x = ;");
        assert!(result.is_err());
        
        // Invalid variable names
        let result = parse_with_error("let 123invalid = 5;");
        assert!(result.is_err());
        
        let result = parse_with_error("let var-name = 5;");
        assert!(result.is_err());
    }

    #[test]
    /// Test various invalid constant declaration syntax patterns
    fn test_invalid_constant_syntax() {
        let result = parse_with_error("const = 5;");
        assert!(result.is_err());
        
        let result = parse_with_error("const X;");
        assert!(result.is_err());
        
        // Invalid constant names
        let result = parse_with_error("const 123INVALID = 5;");
        assert!(result.is_err());
    }

    #[test]
    /// Test malformed string literals
    fn test_malformed_strings() {
        let result = parse_with_error("let x = \"unterminated string;");
        assert!(result.is_err());
        
        let result = parse_with_error("let x = 'unterminated string;");
        assert!(result.is_err());
    }
}