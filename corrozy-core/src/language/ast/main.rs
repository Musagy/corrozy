use crate::language::features::{block::ast::Block, expression::ast::Expression, if_else::ast::ElseClause, loop_for::ast::ForInit};

#[derive(Debug, Clone)]
pub enum AstNode {
    Program { statements: Vec<AstNode> },
    ExpressionStatement {
        expression: Box<Expression>
    },
    VariableDeclaration {
        var_type: Option<String>,
        name: String, 
        value: Box<Expression> 
    },
    ConstantDeclaration {
        name: String,
        const_type: Option<String>,
        value: Box<Expression>
    },
    PrintStatement { 
        expression: Box<Expression>,
        newline: bool
    },
    FunctionDeclaration { 
        name: String, 
        params: Vec<Parameter>, 
        return_type: Option<String>,
        body: Box<Block>
    },
    IfStatement {
        condition: Box<Expression>,
        then_block: Box<Block>,
        else_clause: Option<Box<ElseClause>>
    },
    WhileLoop {
        condition: Box<Expression>,
        body: Box<Block>
    },
    ForLoop {
        init: Option<Box<ForInit>>,
        condition: Option<Box<Expression>>,
        update: Option<Box<Expression>>,
        body: Box<Block>
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<String>,
}
