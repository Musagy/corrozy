use crate::language::{AstNode, features::expression::ast::Expression};

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<AstNode>,
    pub return_statement: Option<ReturnStatement>,
}

impl Block {
    pub fn new() -> Self {
        Block {
            statements: Vec::new(),
            return_statement: None,
        }
    }
}


#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub expression: Option<Box<Expression>>,
}
