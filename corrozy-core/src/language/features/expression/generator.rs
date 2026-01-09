use std::rc::Rc;

use anyhow::{Ok, Result};

use crate::{Config, codegen::CodeGenerator, language::features::{closure::{ClosureGenerator, ast::ClosureBody}, expression::ast::Expression, function::generator::FunctionGenerator, function_call::{generator::FnCallGen}, postfix::generator::PostfixGen}};

pub struct ExpressionGen {
    closure_gen: ClosureGenerator,
    function_gen: FunctionGenerator,
    postfix_gen: PostfixGen,
    function_call_gen: FnCallGen,
}

impl ExpressionGen {
    pub fn new(config: Rc<Config>) -> Self {
        Self {
            closure_gen: ClosureGenerator::new(),
            function_gen: FunctionGenerator::new(config),
            postfix_gen: PostfixGen::new(),
            function_call_gen: FnCallGen::new(),
        }
    }

    pub fn generate(
        &self,
        expr: &Expression,
        code_gen_opt: Option<&CodeGenerator>, 
    ) -> Result<String> {
        match expr {
            Expression::Literal(lit) => {
                Ok(lit.to_php())
            }
            Expression::Variable(name) => {
                Ok(format!("${}", name))
            }
            Expression::BinaryOp { left, op, right } => {
                let left_php = self.generate(left, None)?;
                let right_php = self.generate(right, None)?;
                Ok(format!("{} {} {}", left_php, op.to_php(), right_php))
            }
            Expression::Parenthesized(inner) => {
                let inner_php = self.generate(inner, None)?;
                Ok(format!("({})", inner_php))
            }
            Expression::FunctionCall( function_call_exp) => {
                self.function_call_gen.generate(function_call_exp, self)
            }
            Expression::PostfixChain { base, suffixes } => {
                self.postfix_gen.generate(base, suffixes, self)
            }
            // Important: This is only for global scope, excluding function bodies. Block_gen handles function bodies.
            Expression::Closure { params, return_type, body } => {
                match body.to_owned() {
                    ClosureBody::Block(block) => {
                        let code_gen = code_gen_opt
                                    .as_ref()
                                    .ok_or_else(|| anyhow::anyhow!(
                                        "Code generator is required for block closures"
                                    ))?;
                        let result = self.function_gen.generate_fn_headless(params, block.as_ref(), self, code_gen)?;
                        
                        Ok(result)
                    }
                    ClosureBody::Expression( .. ) => {
                        let result = self.closure_gen.generate(None, params, return_type,                             body, None)?;

                        Ok(result)
                    }
                }
            }
            _ => {
                Err(anyhow::anyhow!("Unsupported expression type for PHP generation"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{codegen::CodeGenerator, language::parser::CorrozyParserImpl, utils::test_utils::default_corrozy_config};
    
    #[test]
    fn test_raw_string_literal() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse("'Hello, World!';").unwrap();
        
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();

        assert_eq!(php.trim(), "'Hello, World!';");
    }

    #[test]
    fn test_interpolated_string_literal() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse("\"Hello, $name!\";").unwrap();
        
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();

        assert_eq!(php.trim(), "\"Hello, $name!\";");
    }

    #[test]
    fn test_generates_simple_addition() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse("5 + 3;").unwrap();
        
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();

        assert_eq!(php.trim(), "5 + 3;");
    }

    #[test]
    fn test_generates_multiple_operations() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse("10 - 2 * 3;").unwrap();
        
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();

        assert_eq!(php.trim(), "10 - 2 * 3;");
    }

    #[test]
    fn test_generates_logical_and() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse("true && false;").unwrap();

        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();

        assert_eq!(php.trim(), "true && false;");
    }

    #[test]
    fn test_generates_logical_or() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse("true || false;").unwrap();

        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();

        assert_eq!(php.trim(), "true || false;");
    }

}