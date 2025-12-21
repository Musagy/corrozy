use crate::language::features::{expression::ast::Expression, function_call::ast::FunctionCallExp};

#[derive(Debug, Clone)]
pub enum PostfixSuffix {
    Index(Box<Expression>),
    Property(String),
    MethodCall(FunctionCallExp)
}