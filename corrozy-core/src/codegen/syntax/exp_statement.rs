use anyhow::Result;

use crate::{codegen::syntax::expression::ExpressionGen, parser::ast::{AstNode, Expression}};

pub struct ExpStatementGenerator {
    expr_gen: ExpressionGen,
}

impl ExpStatementGenerator {
    pub fn new(expression_gen: ExpressionGen) -> Self {
        Self {
            expr_gen: expression_gen,
        }
    }

    pub fn generate<F>(&self, expression: &Expression) -> Result<String>
    where
        F: Fn(&AstNode) -> Result<String> + ?Sized,
    {
        let expr_php = self.expr_gen.generate::<F>(expression, None)?;
        Ok(format!("{};", expr_php))
    }
}