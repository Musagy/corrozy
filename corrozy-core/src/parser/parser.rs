use anyhow::{anyhow, Ok, Result};
use pest::Parser;
use pest_derive::Parser;

use crate::parser::ast::{AstNode, BinaryOperator, Block, Expression, Literal, Parameter, ReturnStatement};

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

    fn parse_statement(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        for inner_pair in pair.into_inner() {
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
                _ => {
                    println!("Unknown statement type: {:?}", inner_pair.as_rule());
                }
            }
        }
        Err(anyhow!("Unknown statement type"))
    }

    fn parse_expression(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::binary_expression => {
                    return self.parse_binary_expression(inner_pair);
                }
                Rule::primary_expression => {
                    return self.parse_primary_expression(inner_pair);
                }
                _ => {}
            }
        }
        Err(anyhow!("Unknown expression type"))
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
                    return Ok(self.parse_fn_call(inner_pair)?);
                }
                Rule::expression => {
                    return Ok(Expression::Parenthesized(Box::new(self.parse_expression(inner_pair)?)));
                }
                _ => {}
            }
        }
        Err(anyhow!("Unknown primary expression"))
    }

    fn parse_binary_expression(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
        let mut inner_pairs = pair.into_inner();
        
        let mut left = self.parse_primary_expression(inner_pairs.next().unwrap())?;
        
        while let (Some(op_pair), Some(right_pair)) = (inner_pairs.next(), inner_pairs.next()) {
            let operator = BinaryOperator::from_str(op_pair.as_str())
                .ok_or_else(|| anyhow!("Unknown binary operator: {}", op_pair.as_str()))?;
            let right = self.parse_primary_expression(right_pair)?;
            
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }

    fn parse_literal(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Literal> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::string => {
                    let mut value = inner_pair.as_str().to_string();
                    value.remove(0);
                    value.pop();
                    return Ok(Literal::String(value));
                }
                Rule::integer => {
                    let value = inner_pair.as_str().parse::<i64>()?;
                    return Ok(Literal::Integer(value));
                }
                Rule::float => {
                    let value = inner_pair.as_str().parse::<f64>()?;
                    return Ok(Literal::Float(value));
                }
                Rule::boolean => {
                    let value = inner_pair.as_str() == "true";
                    return Ok(Literal::Boolean(value));
                }
                _ => {}
            }
        }
        Err(anyhow!("Unknown literal type"))
    }

    fn parse_define_type(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<String> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::type_annotation => {
                    return self.parse_type_annotation(inner_pair);
                }
                _ => {}
            }
        }
        Err(anyhow!("No type annotation found")) // o tu tipo de error
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

    fn parse_declaration_declaration(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
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

    fn parse_output_statement(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
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

    fn parse_fn_call(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
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
        
        Ok(Expression::FunctionCall { name, args })
    }
    
    fn parse_function_declaration(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
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
            body:body,
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

    fn parse_parameter(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Parameter> {
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

    fn parse_block(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Block> {
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
    
    fn parse_return_statement(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Option<ReturnStatement>> {
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

    fn parse_expression_statement(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        for inner_pair in pair.into_inner() {
            if inner_pair.as_rule() == Rule::expression {
                return Ok(AstNode::ExpressionStatement {
                    expression: Box::new(self.parse_expression(inner_pair)?),
                });
            }
        }
        Err(anyhow!("Invalid expression statement"))
    }
    
    fn parse_if_statement(&mut self, _pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        todo!("If statements not implemented yet")
    }
    
    fn parse_while_loop(&mut self, _pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        todo!("While loops not implemented yet")
    }
    
    fn parse_for_loop(&mut self, _pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        todo!("For loops not implemented yet")
    }
}