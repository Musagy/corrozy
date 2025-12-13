use std::rc::Rc;

use anyhow::{Ok, Result};

use crate::{codegen::syntax::{declaration::DeclarationGenerator, exp_statement::ExpStatementGenerator, expression::ExpressionGen, function::FunctionGenerator, if_else::{IfElseGenerator}, output::OutputGenerator}, config::Config, parser::ast::AstNode};


pub struct CodeGenerator {
    // config: &'a Config,
    declaration_gen : DeclarationGenerator,
    output_gen: OutputGenerator,
    function_gen: FunctionGenerator,
    exp_statement_gen: ExpStatementGenerator,
    if_else_gen: IfElseGenerator,
    expression_gen: ExpressionGen,
}

impl CodeGenerator {
    pub fn new(config: Rc<Config>) -> Self {
        let function_gen = FunctionGenerator::new(config.clone());
        let expression_gen = ExpressionGen::new(config.clone());

        Self {
            // config,
            declaration_gen: DeclarationGenerator::new(config.clone()),
            output_gen: OutputGenerator::new(config.clone()),
            function_gen,
            exp_statement_gen: ExpStatementGenerator::new(ExpressionGen::new(config.clone())),
            if_else_gen: IfElseGenerator::new(),
            expression_gen,
        }
    }

    pub fn generate<F>(
        &self,
        ast: &[AstNode]
    ) -> Result<String>
    where 
        F: Fn(&AstNode) -> Result<String>
    {
        let mut output = String::new();

        for node in ast {
            output.push_str(&self.generate_node::<F>(node)?);
        }
        
        Ok(output)
    }
    
    fn generate_node<F>(
        &self,
        node: &AstNode
    ) -> Result<String>
    where 
        F: Fn(&AstNode) -> Result<String>,
    {
        let generator = Rc::new(|node: &AstNode| self.generate_node::<F>(node));
        match node {
            AstNode::Program { statements } => {
                let mut result = String::new();
                for stmt in statements {
                    result.push_str(&self.generate_node::<F>(stmt)?);
                }
                Ok(result)
            }

            AstNode::VariableDeclaration { var_type, name, value } => {
                self.declaration_gen.generate::<F>(var_type, name, value, false)
            }
            
            AstNode::ConstantDeclaration { const_type, name, value } => {
                self.declaration_gen.generate::<F>(const_type, name, value, true)
            }
            
            AstNode::PrintStatement { expression, newline } => {
                self.output_gen.generate::<F>(expression, *newline)
            }
            
            AstNode::ExpressionStatement { expression } => {
                self.exp_statement_gen.generate::<F>(expression, None)
            }

            AstNode::FunctionDeclaration {
                name,
                params,
                return_type,
                body
            } => {
                self.function_gen.generate(
                    name,
                    params,
                    return_type,
                    body,
                    &self.expression_gen,
                    generator
                )
            }

            AstNode::IfStatement { condition, then_block, else_clause } => {
                self.if_else_gen.generate(
                    condition,
                    then_block,
                    else_clause,
                    &self.expression_gen,
                    generator
                )
            }

            _ => {
                Ok(format!("// Node don't implemented: {:?}\n", node))
            }
        }

    }
    
}


