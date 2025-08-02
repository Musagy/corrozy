use anyhow::Result;

use crate::{codegen::syntax::block::BlockGenerator, parser::ast::{AstNode, Block, Parameter}, Config};

pub struct FunctionGenerator<'a> {
    config: &'a Config,
    block_gen: BlockGenerator<'a>,
}

impl<'a> FunctionGenerator<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            block_gen: BlockGenerator::new(config),
        }
    }

    pub fn generate<F>(
        &self,
        name: &str,
        params: &[Parameter],
        return_type: &Option<String>,
        body: &Block,
        node_generator: F,
    ) -> Result<String>
    where
        F: Fn(&AstNode) -> Result<String>,
    {
        let mut result = String::new();

        if self.config.transpiler.include_comments {
            result.push_str(&self.generate_doc(params, return_type));
        }

        result.push_str(&format!("function {}(", name));
        let php_params: Vec<String> = params.iter().map(|param| {
            format!("${}", param.name)
        }).collect();
        result.push_str(&php_params.join(", "));        
        result.push_str(") {\n");

        // result.push_str(&self.generate_body(body)?);

        result.push_str(&self.block_gen.generate(body, node_generator)?);

        result.push_str("}\n");

        return Ok(result);
    }
    fn generate_doc (
        &self,
        params: &[Parameter], 
        return_type: &Option<String>,
    ) -> String {
        let mut result = String::new();
        result.push_str("/**\n");

        for param in params {
            let php_type = param.param_type.as_deref().unwrap_or("mixed");
            result.push_str(&format!(" * @param {} ${}\n", php_type, param.name));
        }

        if let Some(ret_type) = return_type {
            result.push_str(&format!(" * @return {}\n", ret_type));
        }

        result.push_str(" */\n");
        result
    }
}