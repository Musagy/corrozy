use crate::language::features::expression::ast::Expression;


#[derive(Debug, Clone)]
pub struct FunctionCallExp {
    pub name: String,
    pub args: Vec<Expression>,
}
