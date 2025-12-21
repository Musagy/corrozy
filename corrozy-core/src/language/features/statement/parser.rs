use anyhow::{anyhow, Result};

use crate::language::{AstNode, parser::{CorrozyParserImpl, Rule}};

impl CorrozyParserImpl { 
    pub fn parse_statement(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        let inner_pair = pair.into_inner().next()
            .ok_or_else(|| anyhow!("Statement is empty"))?;
        
        match inner_pair.as_rule() {
            Rule::variable_declaration | Rule::constant_declaration => {
                return self.parse_declaration_declaration(inner_pair);
            }
            Rule::print_statement | Rule::println_statement => {
                return self.parse_output_statement(inner_pair);
            }
            Rule::function_declaration => {
                return self.parse_function_declaration(inner_pair);
            }
            Rule::expression_statement => {
                return self.parse_expression_statement(inner_pair);
            }
            Rule::if_statement => {
                return self.parse_if_statement(inner_pair);
            }
            Rule::while_loop => {
                return self.parse_while_loop(inner_pair);
            }
            Rule::for_loop => {
                return self.parse_for_loop(inner_pair);
            }
            _ => Err(anyhow!("Unknown statement type: {:?}", inner_pair.as_rule()))
        }
    }
   
}