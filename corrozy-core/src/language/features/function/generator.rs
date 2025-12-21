use std::rc::Rc;

use anyhow::Result;

use crate::{Config, codegen::CodeGenerator, language::{Parameter, features::{block::{ast::Block, generator::BlockGenerator}, expression::generator::ExpressionGen}}};

pub struct FunctionGenerator {
    config: Rc<Config>,
    block_gen: BlockGenerator,
}

impl FunctionGenerator {
    pub fn new(config: Rc<Config>) -> Self {
        Self {
            config,
            block_gen: BlockGenerator::new(),
        }
    }

    pub fn generate(
        &self,
        name: &str,
        params: &[Parameter],
        return_type: &Option<String>,
        body: &Block,
        expression_gen: &ExpressionGen,
        // node_generator: Rc<dyn Fn(&AstNode) -> Result<String>>,
        code_gen: &CodeGenerator, 
    ) -> Result<String> {
        let mut result = String::new();

        if self.config.transpiler.include_comments && (!params.is_empty() || return_type.is_some()) {
            result.push_str(&self.generate_doc(params, return_type));
        }

        result.push_str(&format!("function {}(", name));
        let php_params: Vec<String> = params.iter().map(|param| {
            format!("${}", param.name)
        }).collect();
        result.push_str(&php_params.join(", "));
        result.push_str(") {\n");

        // result.push_str(&self.generate_body(body)?);

        result.push_str(&self.block_gen.generate(body, expression_gen, code_gen)?);

        result.push_str("}\n");

        return Ok(result);
    }
    
    pub fn generate_fn_headless(
        &self,
        params: &[Parameter],
        body: &Block,
        expression_gen: &ExpressionGen,
        // node_generator: Rc<dyn Fn(&AstNode) -> Result<String>>,
        code_gen: &CodeGenerator, 
    ) -> Result<String> {
        let mut result = String::new();

        // params
        result.push_str(&format!("("));
        let php_params: Vec<String> = params.iter().map(|param| {
            format!("${}", param.name)
        }).collect();
        result.push_str(&php_params.join(", "));
        result.push_str(") {\n");

        // body
        result.push_str(&self.block_gen.generate(body, expression_gen, code_gen)?);
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