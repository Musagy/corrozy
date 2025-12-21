use std::rc::Rc;

use anyhow::{Ok, Result};

use crate::{config::Config, language::{AstNode, features::{expression::generator::ExpressionGen, function::generator::FunctionGenerator, if_else::generator::IfElseGenerator, output::generator::OutputGenerator, statement::generator::ExpStatementGenerator, declaration::generator::DeclarationGenerator}}
};


pub struct CodeGenerator {
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
            declaration_gen: DeclarationGenerator::new(config.clone()),
            output_gen: OutputGenerator::new(config.clone()),
            function_gen,
            exp_statement_gen: ExpStatementGenerator::new(ExpressionGen::new(config.clone())),
            if_else_gen: IfElseGenerator::new(),
            expression_gen,
        }
    }

    pub fn generate(
        &self,
        ast: &[AstNode]
    ) -> Result<String> {
        let mut output = String::new();

        for node in ast {
            output.push_str(&self.generate_node(node)?);
        }
        
        Ok(output)
    }
    
    pub fn generate_node(
        &self,
        node: &AstNode
    ) -> Result<String> {
        match node {
            AstNode::Program { statements } => {
                let mut result = String::new();
                for stmt in statements {
                    result.push_str(&self.generate_node(stmt)?);
                }
                Ok(result)
            }

            AstNode::VariableDeclaration { var_type, name, value } => {
                self.declaration_gen.generate(var_type, name, value, false)
            }
            
            AstNode::ConstantDeclaration { const_type, name, value } => {
                self.declaration_gen.generate(const_type, name, value, true)
            }
            
            AstNode::PrintStatement { expression, newline } => {
                self.output_gen.generate(expression, *newline)
            }
            
            AstNode::ExpressionStatement { expression } => {
                self.exp_statement_gen.generate(expression)
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
                    self
                )
            }

            AstNode::IfStatement { condition, then_block, else_clause } => {
                self.if_else_gen.generate(
                    condition,
                    then_block,
                    else_clause,
                    &self.expression_gen,
                    self
                )
            }

            _ => {
                Ok(format!("// Node don't implemented: {:?}\n", node))
            }
        }

    }
}


