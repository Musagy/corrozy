use anyhow::{Ok, Result};
use crate::{codegen::syntax::expression::ExpressionGen, config::Config, parser::ast::Expression};

pub struct DeclarationGenerator<'a> {
    config: &'a Config,
    expression_gen: ExpressionGen,
}

impl<'a> DeclarationGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            expression_gen: ExpressionGen::new(),
        }
    }
    
    pub fn generate(&self, var_type: &Option<String>, name: &str, value: &Expression, is_constant: bool) -> Result<String> {
        let php_value = self.expression_gen.generate(value)?;
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

        if is_constant {
            output.push_str(&format!("const {} = {};\n", name.to_uppercase(), php_value));
        } else {
            output.push_str(&format!("${} = {};\n", name, php_value));
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