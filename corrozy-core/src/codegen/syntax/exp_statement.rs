use anyhow::Result;

use crate::{codegen::syntax::expression::ExpressionGen, parser::ast::Expression};

pub struct ExpStatementGenerator {
    expr_gen: ExpressionGen,
}

impl ExpStatementGenerator {
    pub fn new() -> Self {
        Self {
            expr_gen: ExpressionGen::new(),
        }
    }

    pub fn generate(&self, expression: &Expression) -> Result<String> {
        let expr_php = self.expr_gen.generate(expression)?;
        Ok(format!("{};", expr_php))
    }
}