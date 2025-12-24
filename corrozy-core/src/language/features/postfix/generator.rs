use anyhow::Result;

use crate::language::features::{expression::{ExpressionGen, ast::Expression}, postfix::ast::PostfixSuffix};

pub struct PostfixGen {}

impl PostfixGen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(
        &self,
        base: &Expression,
        suffixes: &Vec<PostfixSuffix>,
        expression_gen: &ExpressionGen,
    ) -> Result<String> {
        let mut result = expression_gen.generate(base, None)?;

        for suffix in suffixes {
            match suffix {
                PostfixSuffix::Index(index_expr) => {
                    let index_php = expression_gen.generate(&index_expr, None)?;
                    result = format!("{}[{}]", result, index_php);
                }
                PostfixSuffix::MethodCall(func_call) => {
                    let name = &func_call.name;
                    let args = &func_call.args;
                    let arg_strs: Vec<String> = args.iter()
                        .map(|arg| expression_gen.generate(arg, None))
                        .collect::<Result<Vec<_>>>()?;
                    result = format!("{}->{}({})", result, name, arg_strs.join(", "));
                }
                PostfixSuffix::Property(prop_name) => {
                    result = format!("{}->{}", result, prop_name);
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{codegen::CodeGenerator, language::parser::CorrozyParserImpl, utils::test_utils::default_corrozy_config};
    
    #[test]
    fn test_postfix_by_index() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse("arr[0];").unwrap();
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();
        assert_eq!(php.trim(), "$arr[0];");
    }

    #[test]
    fn test_postfix_method_call() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse("obj.method(arg1, arg2);").unwrap();
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();
        assert_eq!(php.trim(), "$obj->method($arg1, $arg2);");
    }
    
    #[test]
    fn test_postfix_property_access() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse("obj.property;").unwrap();
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();
        assert_eq!(php.trim(), "$obj->property;");
    }

    #[test]
    fn test_postfix_chain() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse("obj.method1().property[2];").unwrap();
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();
        assert_eq!(php.trim(), "$obj->method1()->property[2];");
    }
}