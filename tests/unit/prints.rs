
#[cfg(test)]
mod prints_declaration_tests {
    use corrozy_core::language::AstNode;

    use crate::utils::test_utils::{extract_literal_value, parse_single_statement};

    #[test]
    /// Test variable declaration with double quoted string
    fn test_println() {
        let ast = parse_single_statement("println(\"Diego\");");
        
        match ast {
            AstNode::PrintStatement { expression, newline } => {
                assert_eq!(extract_literal_value(&expression), "\"Diego\"");
                assert!(newline);
            }
            other => panic!("Expected PrintStatement, got: {:?}", other),   
        }
    }
    
    #[test]
    fn test_print() {
        let ast = parse_single_statement("print('Hello, World!');");
        
        match ast {
            AstNode::PrintStatement { expression, newline } => {
                assert_eq!(extract_literal_value(&expression), "'Hello, World!'");
                assert!(!newline);
            }
            other => panic!("Expected PrintStatement, got: {:?}", other),   
        }
    }
}