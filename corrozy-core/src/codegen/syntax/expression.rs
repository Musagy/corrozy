use std::rc::Rc;

use anyhow::{Ok, Result};

use crate::{Config, codegen::syntax::{closure::ClosureGenerator, function::FunctionGenerator}, parser::ast::{AstNode, ClosureBody, Expression}};

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

    pub fn generate<F>(
        &self,
        expr: &Expression,
        node_generator_opt: Option<Rc<F>>
    ) -> Result<String> 
    where
        F: Fn(&AstNode) -> Result<String> + ?Sized,
    {
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
                    .map(|arg| self.generate::<F>(arg, None))
                    .collect::<Result<Vec<_>>>()?;
                Ok(format!("{}({})", name, arg_strs.join(", ")))
            }
            Expression::BinaryOp { left, op, right } => {
                let left_php = self.generate::<F>(left, None)?;
                let right_php = self.generate::<F>(right, None)?;
                Ok(format!("{} {} {}", left_php, op.to_php(), right_php))
            }
            Expression::Parenthesized(inner) => {
                let inner_php = self.generate::<F>(inner, None)?;
                Ok(format!("({})", inner_php))
            }

            // Important: This is only for global scope, excluding function bodies. Block_gen handles function bodies.
            Expression::Closure { params, return_type, body } => {
                match body.to_owned() {
                    // When the closure body is a block (multi-line), 
                    // and it's not inside a private/local scope, 
                    // it will be handled by the function generator (function_gen).
                    ClosureBody::Block(block) => {
                        let node_generator = node_generator_opt
                                    .as_ref()
                                    .ok_or_else(|| anyhow::anyhow!(
                                        "Node generator is required for block closures"
                                    ))?;

                        let result = self.function_gen.generate_fn_headless::<F>(
                            params,
                            block.as_ref(),
                            self,
                            node_generator.clone()
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