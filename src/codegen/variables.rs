use anyhow::Result;

use crate::{config::Config, parser::ast::{Expression}};

pub struct VariableGenerator<'a> {
    config: &'a Config,
}

impl<'a> VariableGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
    
    pub fn generate_variable_declaration(&self, var_type: &str, name: &str, value: &Expression) -> Result<String> {
        let php_value = self.generate_expression_php(value)?;
        let phpdoc_type = self.map_type_to_phpdoc(var_type);
        
        if self.config.transpiler.include_comments {
            Ok(format!(
                "/** @var {} ${} */\n${} = {};\n", 
                phpdoc_type, name, name, php_value
            ))
        } else {
            Ok(format!("${} = {};\n", name, php_value))
        }
    }
    
    fn map_type_to_phpdoc(&self, var_type: &str) -> &'static str {
        match var_type {
            "int" => "int",
            "str" => "string", 
            "bool" => "bool",
            "float" => "float",
            "var" => "mixed",
            _ => "mixed"
        }
    }
    
    fn generate_expression_php(&self, expr: &Expression) -> Result<String> {
        match expr {
            Expression::Literal(lit) => {
                Ok(lit.to_php())
            }
            Expression::Variable(name) => {
                Ok(format!("${}", name))
            }
            Expression::FunctionCall { name, args } => {
                let arg_strs: Vec<String> = args.iter()
                    .map(|arg| self.generate_expression_php(arg))
                    .collect::<Result<Vec<_>>>()?;
                Ok(format!("{}({})", name, arg_strs.join(", ")))
            }
            Expression::BinaryOp { left, op, right } => {
                let left_php = self.generate_expression_php(left)?;
                let right_php = self.generate_expression_php(right)?;
                Ok(format!("{} {} {}", left_php, op.to_php(), right_php))
            }
            Expression::Parenthesized(inner) => {
                let inner_php = self.generate_expression_php(inner)?;
                Ok(format!("({})", inner_php))
            }
        }
    }
}