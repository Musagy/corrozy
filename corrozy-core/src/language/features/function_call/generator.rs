use anyhow::Result;
use crate::language::features::{expression::ExpressionGen, function_call::{ast::FunctionCallExp}};

pub struct FnCallGen {}

impl FnCallGen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(
        &self,
        FunctionCallExp { args, name}: &FunctionCallExp,
        expression_gen: &ExpressionGen,
        
    ) -> Result<String> {
        let arg_strs: Vec<String> = args.iter()
            .map(|arg| expression_gen.generate(arg, None))
            .collect::<Result<Vec<_>>>()?;

        Ok(format!("{}({})", name, arg_strs.join(", ")))
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{codegen::CodeGenerator, language::parser::CorrozyParserImpl, utils::test_utils::default_corrozy_config};


    #[test]
    fn test_function_call_no_args() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse(r#"
            greet();
        "#).unwrap();
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();
        assert!(php.contains("greet()"));
    }

    #[test]
    fn test_function_call_with_args() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse(r#"
            add(1, 2, 3);
        "#).unwrap();
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();
        assert!(php.contains("add(1, 2, 3)"));
    }

    #[test]
    fn test_nested_function_calls() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse(r#"
            outer(inner(5));
        "#).unwrap();
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();
        assert!(php.contains("outer(inner(5))"));
    }
}