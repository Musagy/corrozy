use crate::language::{AstNode, features::expression::ast::Expression};

#[derive(Debug, Clone)]
pub enum ForInit {
    VariableDeclaration(Box<AstNode>),
    Expression(Box<Expression>),
}
