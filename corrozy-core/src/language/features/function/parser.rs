use anyhow::Result;

use crate::language::{AstNode, Parameter, features::{block::ast::Block}, parser::{CorrozyParserImpl, Rule}};


impl CorrozyParserImpl {
    pub fn parse_function_declaration(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        let mut return_type: Option<String> = None;
        let mut name = String::new();
        let mut params = Vec::new();
        let mut body: Block = Block::new();

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::identifier => {
                    name = inner_pair.as_str().to_string();
                }
                Rule::parameter_list => {
                    params = self.parse_parameter_list(inner_pair)?;
                }
                Rule::define_type => { 
                    return_type = Some(self.parse_define_type(inner_pair)?);
                }
                Rule::block => {
                    body = self.parse_block(inner_pair)?; 
                }
                _ => {}
            }
        }

        Ok(AstNode::FunctionDeclaration {
            name,
            params,
            return_type,
            body: Box::new(body),
        })
    }

    pub fn parse_parameter(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Parameter> {
        let mut name = String::new();
        let mut param_type = None;

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::identifier => {
                    name = inner_pair.as_str().to_string();
                }
                Rule::define_type => {
                    param_type = Some(self.parse_define_type(inner_pair)?);
                }
                _ => {}
            }
        }

        Ok(Parameter {
            name,
            param_type,
        })
    }

    fn parse_parameter_list(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Vec<Parameter>> {
        let mut params = Vec::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::parameter => {
                    params.push(self.parse_parameter(inner_pair)?);
                }
                _ => {}
            }
        }

        Ok(params)
    }
}