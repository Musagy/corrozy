use std::rc::Rc;

use anyhow::{Ok, Result};

use crate::{codegen::{syntax::{block::BlockGenerator, expression::ExpressionGen}}, parser::ast::{AstNode, Block, ElseClause, Expression}};

pub struct IfElseGenerator{
    block_gen: BlockGenerator,
}

impl IfElseGenerator {
    pub fn new() -> Self {
        Self {
            block_gen: BlockGenerator::new(),
        }
    }

    pub fn generate<F>(
        &self,
        condition: &Expression,
        then_block: &Block,
        else_clause: &Option<Box<ElseClause>>,
        expression_gen: &ExpressionGen,
        node_generator: Rc<F>
    ) -> Result<String> 
    where
        F: Fn(&AstNode) -> Result<String>,
    {
        let mut result = String::new();

        let condition_str = match condition {
            Expression::Parenthesized(_) => {
                let node_gen: Option<Rc<F>> = None;
                expression_gen.generate(condition, node_gen)?
            }
            _ => {
                let node_gen: Option<Rc<F>> = None;
                let expr_str = expression_gen.generate(condition, node_gen)?;
                format!("({})", expr_str)
            }
        };

        result.push_str(&format!("if {} {{\n", condition_str));
        result.push_str(&self.block_gen.generate(then_block, expression_gen, node_generator.clone())?);
        result.push_str("}");

        if let Some(else_clause) = else_clause {
            result.push_str(&self.else_clause_gen(
                else_clause,
                expression_gen,
                node_generator
            )?);
        }

        Ok(result)
    }

    fn else_clause_gen<F>(
        &self,
        else_clause: &Box<ElseClause>,
        expression_gen: &ExpressionGen,
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
                            expression_gen,
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
                result.push_str(&self.block_gen.generate(body, expression_gen, node_generator.clone())?);
                result.push_str("}");

                Ok(result)
            } 
        }
    }
}