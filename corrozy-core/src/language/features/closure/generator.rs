// use crate::codegen::syntax::expression::ExpressionGen;

use core::panic;
use std::collections::HashSet;

use anyhow::{anyhow, Ok, Result};

use crate::language::{AstNode, Parameter, features::{block::ast::Block, closure::ast::ClosureBody, expression::ast::Expression, function_call::ast::FunctionCallExp, if_else::ast::ElseClause}};

#[derive(Clone)]
pub struct ClosureGenerator;

struct ClosurePreloaded {
    _closure_transpiled: String,
    use_clause: HashSet<String>,
}

impl ClosureGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(
        &self,
        name: Option<&String>,
        params: &Vec<Parameter>,
        return_type: &Option<String>,
        body: &ClosureBody,
        parent_scope_opt: Option<&Block>
    ) -> Result<String> {
        let mut result = String::new();

        if name.is_some() {
            result.push_str(format!("${} = ", name.unwrap()).as_str());
        }
        
        match body {
            ClosureBody::Expression(_expr) => {
                // Arrow function
                result.push_str("fn(");
                let php_params = self.generate_params(params);
                result.push_str(&php_params);
                result.push_str(")");
                
                if let Some(ret_type) = return_type {
                    result.push_str(&format!(": {}", ret_type));
                }
                
                result.push_str(" => ");
                // result.push_str(&self.generate_expression(expr));
                result.push_str("EXPRESSION_PLACEHOLDER");
            }
            ClosureBody::Block(block) => {
                // Prioritize loading loose closures in the block
                let mut preloaded_closures: Vec<ClosurePreloaded> = vec![];
                let mut required_vars: HashSet<String> = HashSet::new();

                for statement in &block.statements {
                    match statement {
                        AstNode::FunctionDeclaration { name, params, return_type, body } => {
                            let closure_preloaded = self.generate_closure_multiline(
                                Some(name),
                                params,
                                return_type,
                                body,
                                Some(&required_vars)
                            );
                            required_vars.extend(closure_preloaded.use_clause.clone());
                            preloaded_closures.push(closure_preloaded);
                        }
                        AstNode::VariableDeclaration { name, value, ..} => {
                            if let Expression::Closure { params, return_type, body } = value.as_ref() {
                                if let ClosureBody::Block(block) = body {
                                    let closure_preloaded = self.generate_closure_multiline(
                                        Some(name),
                                        &params,
                                        &return_type,
                                        &block,
                                        Some(&required_vars)
                                    );
                                    required_vars.extend(closure_preloaded.use_clause.clone());
                                    preloaded_closures.push(closure_preloaded);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                // none name for alright it have
                let _closure_preloaded_main = self.generate_closure_multiline(
                    None,
                    params,
                    return_type,
                    block,
                    Some(&required_vars)
                );
                let parent_scope = parent_scope_opt.unwrap_or_else(|| {
                    panic!("Parent scope is required to validate closure variables");
                }); 
                let outsider_vars = self.collect_declared_variables_from_block(parent_scope);
                let lost_vars: HashSet<String> = required_vars.difference(&outsider_vars)
                    .cloned()
                    .collect();
                if !lost_vars.is_empty() {
                    let vars_list: Vec<String> = lost_vars.iter().cloned().collect();

                    return Err(anyhow!(
                        "Undefined variables in closure: {}. These variables are used but not declared in the outer scope.", 
                        vars_list.join(", ")
                    ));
                }
            }
        }

        Ok(result)
    }

    fn generate_closure_multiline(
        &self,
        name: Option<&String>,
        params: &Vec<Parameter>,
        return_type: &Option<String>,
        body: &Block,
        inherited_use_vars: Option<&HashSet<String>>
    ) -> ClosurePreloaded {
        let mut result = String::new();

        // Declaration if it's not anonymous
        if name.is_some() {
            result.push_str(format!("${} = ", name.unwrap()).as_str());
        }
        
        // Traditional closure
        result.push_str("function(");
        let php_params = self.generate_params(params);
        result.push_str(&php_params);
        result.push_str(")");
        
        if let Some(ret_type) = return_type {
            result.push_str(&format!(": {}", ret_type));
        }

        let use_vars = self.generate_use_clause(params, body, inherited_use_vars);
        
        if !use_vars.is_empty() {
            let php_use_vars: Vec<String> = use_vars.iter()
                .map(|var| format!("${}", var))
                .collect();
            let use_clause_string = php_use_vars.join(", ");
            
            result.push_str(&format!(" use ({})", use_clause_string));
        }
        result.push_str(" {\n");

        // generate body
        // result.push_str(&self.generate_block(body));

        result.push_str("}\n");

        ClosurePreloaded {
            _closure_transpiled: result,
            use_clause: use_vars
        }
    }

    fn generate_params(&self, params: &Vec<Parameter>) -> String {
        params.iter().map(|param| {
            if let Some(param_type) = &param.param_type {
                format!("{} ${}", param_type, param.name)
            } else {
                format!("${}", param.name)
            }
        }).collect::<Vec<_>>().join(", ")
    }
    
    fn generate_use_clause(&self,
        params: &Vec<Parameter>,
        closure_body: &Block,    
        inherited_use_vars: Option<&HashSet<String>>
    ) -> HashSet<String> {
        let used_vars = self.collect_used_variables_from_block(closure_body, inherited_use_vars);
        
        let mut declared_vars = self.collect_declared_variables_from_block(closure_body);
        for param in params {
            declared_vars.insert(param.name.clone());
        }
        
        let captured_vars: HashSet<String> = used_vars.difference(&declared_vars)
            .cloned()
            .collect();
        
        captured_vars
    }


    fn collect_used_variables_from_block(&self, block: &Block, starting_vars: Option<&HashSet<String>>) -> HashSet<String> {
        let mut used = if let Some(inherited) = starting_vars {
            inherited.clone()
        } else {
            HashSet::new()
        };
        
        for statement in &block.statements {
            self.walk_for_used_variables(statement, &mut used);
        }
        
        if let Some(return_stmt) = &block.return_statement {
            if let Some(expr) = &return_stmt.expression {
                self.walk_for_used_variables_in_expression(expr, &mut used);
            }
        }
        
        used
    }

    fn collect_declared_variables_from_block(&self, block: &Block) -> HashSet<String> {
        let mut declared = HashSet::new();
        
        for statement in &block.statements {
            self.walk_for_declared_variables(statement, &mut declared);
        }
        
        declared
    }

    fn walk_for_used_variables(&self, node: &AstNode, used: &mut HashSet<String>) {
        match node {
            AstNode::ExpressionStatement { expression } => {
                self.walk_for_used_variables_in_expression(expression, used);
            }
            AstNode::VariableDeclaration { value, .. } => {
                self.walk_for_used_variables_in_expression(value, used);
            }
            AstNode::ConstantDeclaration { value, .. } => {
                self.walk_for_used_variables_in_expression(value, used);
            }
            AstNode::PrintStatement { expression, .. } => {
                self.walk_for_used_variables_in_expression(expression, used);
            }
            AstNode::IfStatement { condition, then_block, else_clause } => {
                self.walk_for_used_variables_in_elseif(condition, then_block, else_clause, used);
            }
            AstNode::WhileLoop { condition, body } => {
                self.walk_for_used_variables_in_expression(condition, used);
                self.walk_for_used_variables_in_block(&body, used);
            }
            AstNode::ForLoop { .. } => {
                // TODO: implement when needed
                // self.walk_for_used_variables_in_block(&body, used);
            }
            _ => {
            }
        }
    }

    fn walk_for_used_variables_in_elseif(&self, condition: &Box<Expression>, then_block: &Block, else_clause: &Option<Box<ElseClause>>, used: &mut HashSet<String>) {
        self.walk_for_used_variables_in_expression(condition, used);
        self.walk_for_used_variables_in_block(&then_block, used);
        if else_clause.is_some() {
            match else_clause.clone().unwrap().as_ref() {
                ElseClause::Else(block) => {
                    self.walk_for_used_variables_in_block(&block, used);
                }
                ElseClause::ElseIf(ast_node) => {
                    match ast_node.as_ref() {
                        AstNode::IfStatement { condition, then_block, else_clause  } => {
                            self.walk_for_used_variables_in_elseif(condition, then_block, else_clause, used);
                        },
                        _ => panic!("ElseIf debe contener un IfStatement"),
                        
                    }
                }
            }
        }
    }

    fn walk_for_used_variables_in_expression(&self, expr: &Expression, used: &mut HashSet<String>) {
        match expr {
            Expression::Variable(name) => {
                used.insert(name.clone());
            }
            Expression::BinaryOp { left, right, .. } => {
                self.walk_for_used_variables_in_expression(left, used);
                self.walk_for_used_variables_in_expression(right, used);
            }
            Expression::FunctionCall(FunctionCallExp {args, name: _}) => {
                for arg in args {
                    self.walk_for_used_variables_in_expression(arg, used);
                }
            }
            Expression::Parenthesized(inner) => {
                self.walk_for_used_variables_in_expression(inner, used);
            }
            Expression::Closure { body, .. } => {
                match body {
                    ClosureBody::Expression(expr) => {
                        self.walk_for_used_variables_in_expression(expr, used);
                    }
                    ClosureBody::Block(block) => {
                        self.walk_for_used_variables_in_block(block, used);
                    }
                }
            }
            _ => {}
        }
    }

    fn walk_for_used_variables_in_block(&self, block: &Block, used: &mut HashSet<String>) {
        for statement in &block.statements {
            self.walk_for_used_variables(statement, used);
        }
        
        if let Some(return_stmt) = &block.return_statement {
            if let Some(expr) = &return_stmt.expression {
                self.walk_for_used_variables_in_expression(expr, used);
            }
        }
    }

    fn walk_for_declared_variables(&self, node: &AstNode, declared: &mut HashSet<String>) {
        match node {
            AstNode::VariableDeclaration { name, .. } => {
                declared.insert(name.clone());
            }
            AstNode::ConstantDeclaration { name, .. } => {
                declared.insert(name.clone());
            }
            AstNode::IfStatement { .. } => {
                // TODO: else_clause
            }
            AstNode::WhileLoop { .. } => {
                // self.walk_for_declared_variables_in_block(&body, declared);
            }
            AstNode::ForLoop { .. } => {
                // self.walk_for_declared_variables_in_block(&body, declared);
            }
            _ => {
            }
        }
    }

    // never used?
    // fn walk_for_declared_variables_in_block(&self, block: &Block, declared: &mut HashSet<String>) {
    //     for statement in &block.statements {
    //         self.walk_for_declared_variables(statement, declared);
    //     }
    // }
}