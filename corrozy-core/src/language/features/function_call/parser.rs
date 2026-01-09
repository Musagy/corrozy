use anyhow::Result;

use crate::language::{features::function_call::ast::FunctionCallExp, parser::{CorrozyParserImpl, Rule}};

impl CorrozyParserImpl {
    pub fn parse_fn_call(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<FunctionCallExp> {
        let mut name = String::new();
        let mut args = Vec::new();
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::identifier => {
                    name = inner_pair.as_str().to_string();
                }
                Rule::argument_list => {
                    for arg_pair in inner_pair.into_inner() {
                        if arg_pair.as_rule() == Rule::expression {
                            args.push(self.parse_expression(arg_pair)?);
                        }
                    }
                }
                _ => {}
            }
        }
        
        Ok(FunctionCallExp{ name, args })
    }
}
// mod test