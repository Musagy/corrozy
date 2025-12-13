use anyhow::{anyhow, Ok, Result};
use pest::Parser;
use pest_derive::Parser;

use crate::parser::ast::AstNode;

#[derive(Parser)]
#[grammar = "grammar/corrozy.pest"]
pub struct CorrozyParser;

pub struct CorrozyParserImpl;

impl CorrozyParserImpl {
    pub fn new() -> Self {
        Self 
    }

    pub fn parse(&mut self,  input: &str) -> Result<Vec<AstNode>> {
        let pairs = CorrozyParser::parse(Rule::program, input)
            .map_err(|e| anyhow!("Parse error: {}", e))?;

        let mut statements = Vec::new();
        
        for pair in pairs {
            match pair.as_rule() {
                Rule::program => {
                    for inner_pair in pair.into_inner() {
                        if inner_pair.as_rule() == Rule::statement {
                            statements.push(self.parse_statement(inner_pair)?);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(statements)
    }


    pub fn parse_expression_statement(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        for inner_pair in pair.into_inner() {
            if inner_pair.as_rule() == Rule::expression {
                return Ok(AstNode::ExpressionStatement {
                    expression: Box::new(self.parse_expression(inner_pair)?),
                });
            }
        }
        Err(anyhow!("Invalid expression statement"))
    }

    // Coming soon
    pub fn parse_while_loop(&mut self, _pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        todo!("While loops not implemented yet")
    }
    
    pub fn parse_for_loop(&mut self, _pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        todo!("For loops not implemented yet")
    }
}