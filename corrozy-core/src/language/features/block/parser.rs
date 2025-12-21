use anyhow::Result;

use crate::language::{features::block::ast::{Block, ReturnStatement}, parser::{CorrozyParserImpl, Rule}};


impl CorrozyParserImpl {
    pub fn parse_block(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Block> {
        let mut statements = Vec::new();
        let mut return_statement = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::statement => {
                    statements.push(self.parse_statement(inner_pair)?);
                }
                Rule::return_statement => {
                    return_statement = self.parse_return_statement(inner_pair)?;
                }
                _ => {

                }
            }
        }

        Ok(Block {
            statements,
            return_statement,
        })
    }
    
    pub fn parse_return_statement(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Option<ReturnStatement>> {
        let mut expression = None;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::expression => {
                    expression = Some(Box::new(self.parse_expression(inner_pair)?));
                }
                _ => {}
            }
        }
    
        Ok(Some(ReturnStatement { expression }))
    }
}


#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::language::{AstNode, features::{block::ast::ReturnStatement, expression::ast::{Expression, Literal, StringType}}, parser::CorrozyParserImpl};
    
    #[test]
    fn test_parse_if_statement_then_block_content() -> Result<()> {
        let input = r#"
            if true {
                println("Hello, World!");
                return 42;
            }
        "#;

        let mut parser = CorrozyParserImpl::new();
        let statements = parser.parse(input)?;
        
        let pair = statements.into_iter().next()
            .expect("El parseo debe retornar al menos un statement");

        match pair {
            AstNode::IfStatement { condition: _, then_block, else_clause } => {
                let block = *then_block;
                
                assert!(else_clause.is_none(), "Expected else_clause to be None");

                assert_eq!(block.statements.len(), 1, "Expected 1 statement (println) in the block");
                assert!(block.return_statement.is_some(), "Expected a return statement (return 42) in the block");

                if let AstNode::PrintStatement { expression, newline } = &block.statements[0] {
                    assert!(newline, "println must have newline=true");
                    assert!(matches!(**expression, 
                        Expression::Literal(Literal::String(StringType::Interpolated(ref s))) if s == "Hello, World!"
                    ), "Primer statement: Expected println(\"Hello, World!\")");
                } else {
                    panic!("Primer statement no es un PrintStatement");
                }

                if let Some(ReturnStatement { expression: Some(ref exp) }) = block.return_statement {
                    assert!(matches!(**exp, 
                        Expression::Literal(Literal::Integer(val)) if val == 42
                    ), "Return statement: Expected expression to be the integer 42");
                } else {
                    panic!("Return statement no fue parseado con la expresión 42");
                }
            }
            _ => panic!("Expected AstNode to be IfStatement"),
        }

        Ok(())
    }
}