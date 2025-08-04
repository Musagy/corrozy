use anyhow::Result;

use crate::parser::ast::Expression;

pub struct ExpressionGen;

impl ExpressionGen {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, expr: &Expression) -> Result<String> {
        match expr {
            Expression::Literal(lit) => {
                Ok(lit.to_php())
            }
            Expression::Variable(name) => {
                Ok(format!("${}", name))
            }
            Expression::FunctionCall { name, args } => {
                let arg_strs: Vec<String> = args.iter()
                    .map(|arg| self.generate(arg))
                    .collect::<Result<Vec<_>>>()?;
                Ok(format!("{}({})", name, arg_strs.join(", ")))
            }
            Expression::BinaryOp { left, op, right } => {
                let left_php = self.generate(left)?;
                let right_php = self.generate(right)?;
                Ok(format!("{} {} {}", left_php, op.to_php(), right_php))
            }
            Expression::Parenthesized(inner) => {
                let inner_php = self.generate(inner)?;
                Ok(format!("({})", inner_php))
            }
            _ => {
                Err(anyhow::anyhow!("Unsupported expression type for PHP generation"))
            }
        }
    }
}