use corrozy_core::parser::ast::{AstNode, Literal, StringType};
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
}

#[cfg(test)]
mod literal_generation_tests {
    use super::*;

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
    /// Test interpolated string (double quotes) generation
    fn test_interpolated_string_generation() {
        let literal = Literal::String(StringType::Interpolated("Hello world".to_string()));
        assert_eq!(literal.to_php(), "\"Hello world\"");
    }

    #[test]
    /// Test raw string (single quotes) generation
    fn test_raw_string_generation() {
        let literal = Literal::String(StringType::Raw("Hello world".to_string()));
        assert_eq!(literal.to_php(), "'Hello world'");
    }

    #[test]
    /// Test interpolated string with PHP variables
    fn test_interpolated_string_with_variables() {
        let literal = Literal::String(StringType::Interpolated("Hello $name".to_string()));
        assert_eq!(literal.to_php(), "\"Hello $name\"");
    }

    #[test]
    /// Test raw string with PHP variables (no interpolation)
    fn test_raw_string_with_variables() {
        let literal = Literal::String(StringType::Raw("Hello $name".to_string()));
        assert_eq!(literal.to_php(), "'Hello $name'");
    }

    #[test]
    /// Test interpolated string with mixed content
    fn test_interpolated_string_mixed_content() {
        let literal = Literal::String(StringType::Interpolated("Price: $price for item".to_string()));
        assert_eq!(literal.to_php(), "\"Price: $price for item\"");
    }
}

#[cfg(test)]
mod syntax_error_tests {
    use super::*;

    #[test]
    #[should_panic]
    /// Test that missing semicolon causes parser error
    fn test_missing_semicolon() {
        parse_single_statement("let x = 1");
    }

    #[test]
    #[should_panic]
    /// Test that missing assignment in variable declaration causes parser error
    fn test_missing_variable_assignment() {
        parse_single_statement("let x;");
    }

    #[test]
    #[should_panic]
    /// Test that invalid type annotation causes parser error
    fn test_invalid_type() {
        parse_single_statement("let x: invalide = 1;");
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
    }

    #[test]
    /// Test various invalid constant declaration syntax patterns
    fn test_invalid_constant_syntax() {
        let result = parse_with_error("const = 5;");
        assert!(result.is_err());
        
        let result = parse_with_error("const X;");
        assert!(result.is_err());
    }
}