use anyhow::{Ok, Result};

use crate::{codegen::{output::OutputGenerator, variables::VariableGenerator}, config::Config, parser::ast::{AstNode}};


pub struct CodeGenerator<'a> {
    config: &'a Config,
    variable_gen: VariableGenerator<'a>,
    output_gen: OutputGenerator,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            variable_gen: VariableGenerator::new(config),
            output_gen: OutputGenerator::new(),
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
            AstNode::VariableDeclaration { var_type, name, value } => {
                self.variable_gen.generate_variable_declaration(var_type, name, value)
            }
            
            AstNode::PrintStatement { expression, newline } => {
                self.output_gen.generate_print(expression, *newline)
            }
            
            AstNode::ExpressionStatement { .. } => {
                Ok(String::new())
            }
            
            AstNode::Program { statements } => {
                let mut result = String::new();
                for stmt in statements {
                    result.push_str(&self.generate_node(stmt)?);
                }
                Ok(result)
            }

            _ => {
                Ok(format!("// Node don't implemented: {:?}\n", node))
            }
        }

    }
    
}