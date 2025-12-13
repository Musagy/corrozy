
use anyhow::{anyhow, Result};

use crate::parser::{ast::{AstNode, Block, ElseClause, Expression}, parser::Rule};

use super::parser::CorrozyParserImpl;

impl CorrozyParserImpl {
    pub fn parse_if_statement(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        let mut condition: Option<Box<Expression>> = None;
        let mut then_block: Option<Box<Block>> = None;
        let mut else_clause: Option<Box<ElseClause>> = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::expression => {
                    condition = Some(Box::new(self.parse_expression(inner_pair)?));
                }
                Rule::block => {
                    if then_block.is_none() {
                        then_block = Some(Box::new(self.parse_block(inner_pair)?));
                    }
                }
                Rule::else_clause => {
                    else_clause = Some(Box::new(self.parse_else_clause(inner_pair)?));
                }
                _ => {}
            }
        }

        Ok(AstNode::IfStatement {
            condition: condition.expect("Grammar guarantees condition exists"),
            then_block: then_block.expect("Grammar guarantees then_block exists"),
            else_clause,
        })
    }

    fn parse_else_clause(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<ElseClause> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::if_statement => {
                    let if_node = self.parse_if_statement(inner_pair)?;
                    return Ok(ElseClause::ElseIf(Box::new(if_node)));
                }
                Rule::block => {
                    return Ok(ElseClause::Else(Box::new(self.parse_block(inner_pair)?)));
                }
                _ => {}
            }
        }
        Err(anyhow!("Empty else clause"))
    }
}