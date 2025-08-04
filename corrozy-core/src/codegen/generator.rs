use anyhow::{Ok, Result};

use crate::{codegen::syntax::{exp_statement::ExpStatementGenerator, function::FunctionGenerator, output::OutputGenerator, variables::VariableGenerator}, config::Config, parser::ast::AstNode};


pub struct CodeGenerator<'a> {
    // config: &'a Config,
    variable_gen: VariableGenerator<'a>,
    output_gen: OutputGenerator,
    function_gen: FunctionGenerator<'a>,
    exp_statement_gen: ExpStatementGenerator,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            // config,
            variable_gen: VariableGenerator::new(config),
            output_gen: OutputGenerator::new(),
            function_gen: FunctionGenerator::new(config),
            exp_statement_gen: ExpStatementGenerator::new(),
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
        match node {
            AstNode::Program { statements } => {
                let mut result = String::new();
                for stmt in statements {
                    result.push_str(&self.generate_node(stmt)?);
                }
                Ok(result)
            }

            AstNode::VariableDeclaration { var_type, name, value } => {
                self.variable_gen.generate(var_type, name, value)
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
                    |node| self.generate_node(node) // Closure que captura self
                )
            }

            _ => {
                Ok(format!("// Node don't implemented: {:?}\n", node))
            }
        }

    }
    
}


