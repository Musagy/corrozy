use anyhow::Result;

use crate::language::features::expression::{ast::Expression, generator::ExpressionGen};

pub struct ExpStatementGenerator {
    expr_gen: ExpressionGen,
}

impl ExpStatementGenerator {
    pub fn new(expression_gen: ExpressionGen) -> Self {
        Self {
            expr_gen: expression_gen,
        }
    }

    pub fn generate(&self, expression: &Expression) -> Result<String> {
        let expr_php = self.expr_gen.generate(expression, None)?;
        Ok(format!("{};", expr_php))
    }
}