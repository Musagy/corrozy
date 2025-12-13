use anyhow::{anyhow, Result};

use crate::parser::{ast::{BinaryOperator, Expression, Literal, StringType}, parser::Rule};

use super::parser::CorrozyParserImpl;

impl CorrozyParserImpl {

    pub fn parse_expression(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
        // expression = { binary_expression_or_single_postfix }
        let inner_pair = pair.into_inner().next()
            .ok_or_else(|| anyhow!("Expression is empty"))?;

        match inner_pair.as_rule() {
            Rule::binary_expression_or_single_postfix => {
                self.parse_binary_or_postfix_expression(inner_pair)
            }
            _ => Err(anyhow!("Unexpected rule inside expression: {:?}", inner_pair.as_rule()))
        }
    }

    pub fn parse_binary_or_postfix_expression(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
        let mut inner_pairs = pair.into_inner();
        
        let mut left = self.parse_postfix_expression(inner_pairs.next().unwrap())?;
        
        while let (Some(op_pair), Some(right_pair)) = (inner_pairs.next(), inner_pairs.next()) {
            let operator = BinaryOperator::from_str(op_pair.as_str())
                .ok_or_else(|| anyhow!("Unknown binary operator: {}", op_pair.as_str()))?;
            
            let right = self.parse_postfix_expression(right_pair)?;
            
            left = Expression::BinaryOp {
                left: Box::new(left),
                op: operator,
                right: Box::new(right),
            };
        }
        
        Ok(left)
    }
    
    pub fn parse_literal(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Literal> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::string => {
                    return self.parse_string_literal(inner_pair);
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

    pub fn parse_string_literal(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Literal> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::interpolated_string => {
                    let mut value = inner_pair.as_str().to_string();
                    value.remove(0); // Quita "
                    value.pop();     // Quita "
                    return Ok(Literal::String(StringType::Interpolated(value)));
                }
                Rule::raw_string => {
                    let mut value = inner_pair.as_str().to_string();
                    value.remove(0); // Quita '
                    value.pop();     // Quita '
                    return Ok(Literal::String(StringType::Raw(value)));
                }
                _ => {}
            }
        }
        Err(anyhow!("Unknown string type"))
    }

}