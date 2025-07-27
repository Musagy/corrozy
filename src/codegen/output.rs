use crate::parser::ast::*;
use anyhow::Result;

pub struct OutputGenerator;

impl OutputGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_print(&self, expression: &Expression, newline: bool) -> Result<String> {
        let php_expr = self.generate_expression_php(expression)?;
        
        if newline {
            Ok(format!("echo {} . \"\\n\";\n", php_expr))
        } else {
            Ok(format!("echo {};\n", php_expr))
        }
    }
    
    fn generate_expression_php(&self, expr: &Expression) -> Result<String> {
        match expr {
            Expression::Literal(lit) => {
                Ok(lit.to_php()) // Usamos el método que ya definimos en el AST
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