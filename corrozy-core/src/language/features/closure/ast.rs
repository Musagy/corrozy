use crate::language::features::{block::ast::Block, expression::ast::Expression};

#[derive(Debug, Clone)]
pub enum ClosureBody {
    Expression(Box<Expression>),
    Block(Box<Block>),
}
