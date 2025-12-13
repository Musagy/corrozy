use std::rc::Rc;

use anyhow::{Ok, Result};
use crate::{codegen::syntax::expression::ExpressionGen, config::Config, parser::ast::{AstNode, ClosureBody, Expression}};

pub struct DeclarationGenerator {
    config: Rc<Config>,
    expression_gen: ExpressionGen,
}

impl DeclarationGenerator {
    pub fn new(_config: Rc<Config>) -> Self {
        Self {
            config: _config.clone(),
            expression_gen: ExpressionGen::new(_config.clone()),
        }
    }
    
    pub fn generate<F>(
        &self, var_type: &Option<String>,
        name: &str,
        value: &Expression,
        is_constant: bool
    ) -> Result<String> 
    where
        F: Fn(&AstNode) -> Result<String> + ?Sized,
    {
        let php_value = self.expression_gen.generate::<F>(value, None)?;
        let mut output = String::new();
        
        if self.config.transpiler.include_comments {
            if let Some(type_str) = var_type {
                let phpdoc_type = self.map_type_to_phpdoc(type_str);
                
                if is_constant {
                    output.push_str(&format!("/** @var {} */\n", phpdoc_type));
                } else {
                    output.push_str(&format!("/** @var {} ${} */\n", phpdoc_type, name));
                }
            }
        }

        match value {
            Expression::Closure { params: _, return_type: _, body } => {
                match body {
                    ClosureBody::Block(..) => {
                        output.push_str(&format!("function {}{}", name, php_value));
                    }
                    ClosureBody::Expression(..) => {

                        output.push_str(&format!("${} = {};\n", name, php_value));
                    }
                }

            }
            _ => {
                if is_constant {
                    output.push_str(&format!("const {} = {};\n", name.to_uppercase(), php_value));
                } else {
                    output.push_str(&format!("${} = {};\n", name, php_value));
                }
            }
        }

        
        Ok(output)
    }
    
    fn map_type_to_phpdoc(&self, var_type: &str) -> &'static str {
        match var_type {
            "int" => "int",
            "string" => "string", 
            "bool" => "bool",
            "float" => "float",
            "var" => "mixed",
            _ => "mixed"
        }
    }
}