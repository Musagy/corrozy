use std::rc::Rc;

use crate::{Config, language::features::expression::{ast::Expression, generator::ExpressionGen}};
use anyhow::Result;

pub struct OutputGenerator {
    expression_gen: ExpressionGen,
}

impl OutputGenerator {
    pub fn new(config: Rc<Config>) -> Self {
        Self {
            expression_gen: ExpressionGen::new(config),
        }
    }
    
    pub fn generate(
        &self,
        expression: &Expression,
        newline: bool
    ) -> Result<String> {
        let php_expr = self.expression_gen.generate(expression, None)?;

        if newline {
            Ok(format!("echo {} . \"\\n\";\n", php_expr))
        } else {
            Ok(format!("echo {};\n", php_expr))
        }
    }
}