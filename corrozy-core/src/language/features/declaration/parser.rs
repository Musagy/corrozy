use anyhow::{Result, anyhow};

use crate::language::{AstNode, features::{expression::ast::Expression, postfix::ast::PostfixSuffix}, parser::{CorrozyParserImpl, Rule}};

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
    
    fn parse_primary_expression(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::literal => {
                    return Ok(Expression::Literal(self.parse_literal(inner_pair)?));
                }
                Rule::identifier => {
                    return Ok(Expression::Variable(inner_pair.as_str().to_string()));
                }
                Rule::function_call => {
                    return Ok(Expression::FunctionCall(self.parse_fn_call(inner_pair)?));
                }
                Rule::expression => {
                    return Ok(Expression::Parenthesized(Box::new(self.parse_expression(inner_pair)?)));
                }
                _ => {
                    return Err(anyhow!("Unknown primary expression rule: {:?}", inner_pair.as_rule()));
                }
            }
        }
        Err(anyhow!("Unknown primary expression"))
    }

    pub fn parse_define_type(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<String> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::type_annotation => {
                    return self.parse_type_annotation(inner_pair);
                }
                _ => {}
            }
        }
        Err(anyhow!("No type annotation found"))
    }

    fn parse_type_annotation(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<String> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::basic_type | Rule::custom_type => {
                    return Ok(inner_pair.as_str().to_string());
                }
                _ => {}
            }
        }
        Err(anyhow!("Invalid type annotation"))
    }

    pub fn parse_declaration_declaration(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        let is_constant = pair.as_rule() == Rule::constant_declaration;
        let mut var_type: Option<String> = None;
        let mut name = String::new();
        let mut value = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::define_type => {
                    var_type = Some(self.parse_define_type(inner_pair)?);
                }
                Rule::identifier => {
                    name = inner_pair.as_str().to_string();
                }
                Rule::expression => {
                    value = Some(Box::new(self.parse_expression(inner_pair)?));
                }
                _ => {}
            }
        }
        
        let val = value.ok_or_else(|| anyhow!("Declaration missing value"))?;
        
        if is_constant {
            Ok(AstNode::ConstantDeclaration { name, const_type: var_type, value: val })
        } else {
            Ok(AstNode::VariableDeclaration { var_type, name, value: val })
        }
    }
}
