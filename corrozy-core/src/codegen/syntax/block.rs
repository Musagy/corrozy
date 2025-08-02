use anyhow::Result;

use crate::{codegen::syntax::expression::ExpressionGen, parser::ast::{AstNode, Block}};


pub struct BlockGenerator {
    expression_gen: ExpressionGen,
}

impl BlockGenerator {
    pub fn new() -> Self {
        Self {
            expression_gen: ExpressionGen::new(),
        }
    }

    pub fn generate<F>(&self, block: &Block, node_generator: F) -> Result<String>
    where
        F: Fn(&AstNode) -> Result<String>,
    {
        let mut result = String::new();

        for statement in &block.statements {
            let generated = node_generator(statement)?;
            
            for line in generated.lines() {
                if !line.trim().is_empty() { // Solo indentar líneas no vacías
                    result.push_str("    ");
                    result.push_str(line);
                }
                result.push('\n');
            }
        }

        // Generar return statement
        if let Some(return_stmt) = &block.return_statement {
            result.push_str("    ");

            match &return_stmt.expression {
                Some(expr) => {
                    let return_code = self.expression_gen.generate(expr)?;
                    result.push_str(&format!("return {};\n", return_code));
                }
                None => {
                    result.push_str("return;\n");
                }
            }
            // let return_expr = node_generator(return_stmt)?;
            // result.push('\n');
        }

        Ok(result)
    }
}