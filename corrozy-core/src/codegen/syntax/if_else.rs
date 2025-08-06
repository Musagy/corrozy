use std::rc::Rc;

use anyhow::{Ok, Result};

use crate::{codegen::{syntax::{block::BlockGenerator, expression::ExpressionGen}}, parser::ast::{AstNode, Block, ElseClause, Expression}};

pub struct IfElseGenerator{
    block_gen: BlockGenerator,
    expression_gen: ExpressionGen,
}

impl IfElseGenerator {
    pub fn new() -> Self {
        Self {
            block_gen: BlockGenerator::new(),
            expression_gen: ExpressionGen::new(),
        }
    }

    pub fn generate<F>(
        &self,
        condition: &Expression,
        then_block: &Block,
        else_clause: &Option<Box<ElseClause>>, 
        node_generator: Rc<F>
    ) -> Result<String> 
    where
        F: Fn(&AstNode) -> Result<String>,
    {
        let mut result = String::new();

        let condition_str = match condition {
            Expression::Parenthesized(_) => {
                self.expression_gen.generate(condition)?
            }
            _ => {
                let expr_str = self.expression_gen.generate(condition)?;
                format!("({})", expr_str)
            }
        };

        result.push_str(&format!("if {} {{\n", condition_str));
        result.push_str(&self.block_gen.generate(then_block, node_generator.clone())?);
        result.push_str("}");

        if let Some(else_clause) = else_clause {
            result.push_str(&self.else_clause_gen(
                else_clause,
                node_generator
            )?);
        }

        Ok(result)
    }

    fn else_clause_gen<F>(
        &self,
        else_clause: &Box<ElseClause>, 
        node_generator: Rc<F>
    ) -> Result<String> 
    where
        F: Fn(&AstNode) -> Result<String>,
    {
        let mut result = String::new();

        result.push_str(" else");
        match else_clause.as_ref() { // ✅ Agregado as_ref()
            ElseClause::ElseIf(ast_node) => { // ✅ Nombre correcto de variable
                match ast_node.as_ref() { // ✅ Agregado as_ref()
                    AstNode::IfStatement { condition, then_block, else_clause } => {
                        let raw = self.generate(
                            condition,
                            then_block,
                            else_clause,
                            node_generator.clone()
                        )?;
                        result.push_str(&raw);

                        Ok(result)
                    }
                    _ => {
                        Err(anyhow::anyhow!("Invalid ElseClause type"))
                    }
                }
            } 
            ElseClause::Else (body) => {
                result.push_str(" {\n");
                result.push_str(&self.block_gen.generate(body, node_generator.clone())?);
                result.push_str("}");

                Ok(result)
            } 
        }
    }
}