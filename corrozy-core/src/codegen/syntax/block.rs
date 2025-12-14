use anyhow::{Ok, Result};

use crate::{codegen::{CodeGenerator, syntax::{closure::ClosureGenerator, expression::ExpressionGen}}, parser::ast::{AstNode, Block, ClosureBody, Expression}};

pub struct BlockGenerator {
    closure_gen: ClosureGenerator
}

impl BlockGenerator {
    pub fn new() -> Self {
        Self {
            closure_gen: ClosureGenerator::new()
        }
    }

    pub fn generate(
        &self,
        block: &Block,
        expression_gen: &ExpressionGen,
        // node_generator: Rc<dyn Fn(&AstNode) -> Result<String>>,
        code_gen: &CodeGenerator, 
    ) -> Result<String> {
        let mut result = String::new();

        for statement in &block.statements {
            let generated = self.generate_statement(
                statement,
                block,
                expression_gen,
                code_gen
            )?;
            
            for line in generated.lines() {
                if !line.trim().is_empty() {
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
                    let return_code = expression_gen.generate(expr, None)?;
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

    fn generate_statement(
        &self,
        statement: &AstNode,
        block: &Block,
        _expression_gen: &ExpressionGen,
        // node_generator: Rc<dyn Fn(&AstNode) -> Result<String>>,
        code_gen: &CodeGenerator, 
    ) -> Result<String> {
        let mut generated = String::new();
        
        match statement {
            AstNode::FunctionDeclaration {
                name,
                params,
                return_type,
                body
            } => {
                let closure_raw = self.closure_gen.generate(Some(name), params, return_type, &ClosureBody::Block(body.clone()), Some(block))?;
                generated.push_str(&closure_raw);
                generated.push_str("\n");
            }
            AstNode::VariableDeclaration { name, value , .. } => {
                match value.as_ref() {
                    Expression::Closure { params, return_type, body } => {
                        let closure_raw = self.closure_gen.generate(Some(name), params, return_type, body, Some(block))?;
                        generated.push_str(&closure_raw);
                        generated.push_str("\n");
                    }
                    _ => {
                        generated.push_str(code_gen.generate_node(statement)?.as_str());
                    }
                }
            }
            AstNode::ExpressionStatement { expression } => {
                match expression.as_ref() {
                    Expression::Closure { params, return_type, body } => {
                        let closure_raw = self.closure_gen.generate(None, params, return_type, body, Some(block))?;
                        generated.push_str(&closure_raw);
                        generated.push_str("\n");
                    }
                    _ => {
                        generated.push_str(code_gen.generate_node(statement)?.as_str());
                    }
                }
            }
            _ => {
                generated.push_str(code_gen.generate_node(statement)?.as_str());
            }
        };
        Ok(generated)
    }
}