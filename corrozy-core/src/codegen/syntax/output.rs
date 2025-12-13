use std::rc::Rc;

use crate::{Config, codegen::syntax::expression::ExpressionGen, parser::ast::*};
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
    
    pub fn generate<F>(
        &self,
        expression: &Expression,
        newline: bool
    ) -> Result<String> 
    where 
        F: Fn(&AstNode) -> Result<String>
    {
        let php_expr = self.expression_gen.generate::<F>(expression, None)?;

        if newline {
            Ok(format!("echo {} . \"\\n\";\n", php_expr))
        } else {
            Ok(format!("echo {};\n", php_expr))
        }
    }
}