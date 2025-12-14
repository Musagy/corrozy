use std::rc::Rc;

use anyhow::{Ok, Result};

use crate::{Config, codegen::{CodeGenerator, syntax::{closure::ClosureGenerator, function::FunctionGenerator}}, parser::ast::{ClosureBody, Expression}};

pub struct ExpressionGen {
    closure_gen: ClosureGenerator,
    function_gen: FunctionGenerator,
}

impl ExpressionGen {
    pub fn new(config: Rc<Config>) -> Self {
        Self {
            closure_gen: ClosureGenerator::new(),
            function_gen: FunctionGenerator::new(config),
        }
    }

    pub fn generate(
        &self,
        expr: &Expression,
        // node_generator_opt: Option<Rc<dyn Fn(&AstNode) -> Result<String>>>

        code_gen_opt: Option<&CodeGenerator>, 
    ) -> Result<String> {
        // let node_gen: Option<Rc<F>> = None;

        match expr {
            Expression::Literal(lit) => {
                Ok(lit.to_php())
            }
            Expression::Variable(name) => {
                Ok(format!("${}", name))
            }
            Expression::FunctionCall(function_call_exp) => {
                
                let name = &function_call_exp.name;
                let args = &function_call_exp.args;
                let arg_strs: Vec<String> = args.iter()
                    .map(|arg| self.generate(arg, None))
                    .collect::<Result<Vec<_>>>()?;
                Ok(format!("{}({})", name, arg_strs.join(", ")))
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

            // Important: This is only for global scope, excluding function bodies. Block_gen handles function bodies.
            Expression::Closure { params, return_type, body } => {
                match body.to_owned() {
                    // When the closure body is a block (multi-line), 
                    // and it's not inside a private/local scope, 
                    // it will be handled by the function generator (function_gen).
                    ClosureBody::Block(block) => {
                        let code_gen = code_gen_opt
                                    .as_ref()
                                    .ok_or_else(|| anyhow::anyhow!(
                                        "Code generator is required for block closures"
                                    ))?;

                        let result = self.function_gen.generate_fn_headless(
                            params,
                            block.as_ref(),
                            self,
                            code_gen
                        )?;

                        Ok(result)
                    }


                    // When the closure body is a single expression, 
                    // it uses the common expression generator.
                    ClosureBody::Expression( .. ) => {
                        let result = self.closure_gen.generate(
                            None,
                            params,
                            return_type,
                            body,
                            None
                        )?;
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