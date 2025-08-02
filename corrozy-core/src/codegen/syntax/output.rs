use crate::{codegen::syntax::expression::ExpressionGen, parser::ast::*};
use anyhow::Result;

pub struct OutputGenerator {
    expression_gen: ExpressionGen,
}

impl OutputGenerator {
    pub fn new() -> Self {
        Self {
            expression_gen: ExpressionGen::new(),
        }
    }
    
    pub fn generate(&self, expression: &Expression, newline: bool) -> Result<String> {
        let php_expr = self.expression_gen.generate(expression)?;
        // (expression)?;
        
        if newline {
            Ok(format!("echo \"{}\\n\";\n", php_expr))
        } else {
            Ok(format!("echo {};\n", php_expr))
        }
    }
    
}