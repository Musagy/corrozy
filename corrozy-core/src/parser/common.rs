use anyhow::{anyhow, Result};

use crate::parser::{ast::{AstNode, Block, ReturnStatement}, parser::Rule};

use super::parser::CorrozyParserImpl;

impl CorrozyParserImpl {
    pub fn parse_output_statement(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
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