use anyhow::{Result, anyhow};
use pest::iterators::Pair;

use crate::language::{AstNode, parser::{CorrozyParserImpl, Rule}};


impl CorrozyParserImpl {
    pub fn parse_output_statement(&mut self, pair: Pair<Rule>) -> Result<AstNode> {
        let is_println = pair.as_rule() == Rule::println_statement;
        
        for inner_pair in pair.into_inner() {
            if inner_pair.as_rule() == Rule::expression {
                return Ok(AstNode::PrintStatement {
                    expression: Box::new(self.parse_expression(inner_pair)?),
                    newline: is_println,
                });
            }
        }
        Err(anyhow!("Invalid output statement"))
    }
}


#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::language::{AstNode, features::expression::ast::{Expression, Literal, StringType}, parser::CorrozyParserImpl};
    
    #[test]
    fn test_parse_output_statement_println_string_literal() -> Result<()> {
        let input = "println(\"123\");"; 
        
        let mut parser = CorrozyParserImpl::new(); 
        
        let statements = parser.parse(input)?; 
        let pair = statements.into_iter().next()
            .expect("Parsed statements should not be empty");

        match pair {
            AstNode::PrintStatement { expression, newline } => {
                assert!(newline, "Expected newline to be true for println");

                match *expression {
                    Expression::Literal(literal) => {
                        match literal {
                            Literal::String(StringType::Interpolated(value)) => {
                                assert_eq!(value, "123", "Expected literal string value to be '123'");
                            }
                            Literal::String(StringType::Raw(_)) => {
                                panic!("Expected StringType::Interpolated, got StringType::Raw. Check grammar or input quotes.");
                            }
                            _ => panic!("Expected literal to be String"),
                        }
                    }
                    _ => panic!("Expected expression to be a Literal"),
                }
            }
            _ => panic!("Expected AstNode to be PrintStatement"),
        }
        
        Ok(())
    }   
}