use crate::language::{AstNode, features::block::ast::Block};

#[derive(Debug, Clone)]
pub enum ElseClause {
    ElseIf(Box<AstNode>), 
    Else(Box<Block>),
}