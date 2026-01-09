use anyhow::{Result, anyhow};

use crate::language::{features::{expression::ast::Expression, postfix::ast::PostfixSuffix}, parser::{CorrozyParserImpl, Rule}};


impl CorrozyParserImpl {
    pub fn parse_postfix_expression(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
        let mut inner_pairs = pair.into_inner();

        let base_pair = inner_pairs.next().ok_or_else(|| anyhow!("Postfix expression missing base"))?;
        let base_expr = self.parse_primary_expression(base_pair)?;

        let mut suffixes: Vec<PostfixSuffix> = Vec::new();
        
        for suffix_pair in inner_pairs {
            match suffix_pair.as_rule() {
                Rule::expression => {
                    let index_expr = self.parse_expression(suffix_pair)?;
                    suffixes.push(PostfixSuffix::Index(Box::new(index_expr)));
                }
                
                Rule::function_call => {
                    let func_call = self.parse_fn_call(suffix_pair)?;
                    suffixes.push(PostfixSuffix::MethodCall(func_call));
                }

                Rule::identifier => {
                    let name = suffix_pair.as_str().to_string();
                    suffixes.push(PostfixSuffix::Property(name));
                }

                _ => return Err(anyhow!("Unexpected rule in postfix: {:?}", suffix_pair.as_rule())),
            }
        }

        if suffixes.is_empty() {
            Ok(base_expr)
        } else {
            Ok(Expression::PostfixChain {
                base: Box::new(base_expr),
                suffixes,
            })
        }
    }
}