use std::rc::Rc;

use anyhow::{Ok, Result};

use crate::{codegen::syntax::{declaration::DeclarationGenerator, exp_statement::ExpStatementGenerator, function::FunctionGenerator, if_else::{IfElseGenerator}, output::OutputGenerator}, config::Config, parser::ast::AstNode};


pub struct CodeGenerator<'a> {
    // config: &'a Config,
    declaration_gen : DeclarationGenerator<'a>,
    output_gen: OutputGenerator,
    function_gen: FunctionGenerator<'a>,
    exp_statement_gen: ExpStatementGenerator,
    if_else_gen: IfElseGenerator
}

impl<'a> CodeGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            // config,
            declaration_gen: DeclarationGenerator::new(config),
            output_gen: OutputGenerator::new(),
            function_gen: FunctionGenerator::new(config),
            exp_statement_gen: ExpStatementGenerator::new(),
            if_else_gen: IfElseGenerator::new(),
        }
    }

    pub fn generate(&self, ast: &[AstNode]) -> Result<String> {
        let mut output = String::new();

        for node in ast {
            output.push_str(&self.generate_node(node)?);
        }
        
        Ok(output)
    }
    
    fn generate_node(&self, node: &AstNode) -> Result<String> {
        let generator = Rc::new(|node: &AstNode| self.generate_node(node));
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
                    generator
                )
            }

            AstNode::IfStatement { condition, then_block, else_clause } => {

                self.if_else_gen.generate(
                    condition,
                    then_block,
                    else_clause,
                    generator
                )
            }

            _ => {
                Ok(format!("// Node don't implemented: {:?}\n", node))
            }
        }

    }
    
}


