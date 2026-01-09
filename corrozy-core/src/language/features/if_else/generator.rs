use anyhow::{Ok, Result};

use crate::{codegen::CodeGenerator, language::{AstNode, features::{block::{ast::Block, generator::BlockGenerator}, expression::{ast::Expression, generator::ExpressionGen}, if_else::ast::ElseClause}}, };

pub struct IfElseGenerator{
    block_gen: BlockGenerator,
}

impl IfElseGenerator {
    pub fn new() -> Self {
        Self {
            block_gen: BlockGenerator::new(),
        }
    }

    pub fn generate(
        &self,
        condition: &Expression,
        then_block: &Block,
        else_clause: &Option<Box<ElseClause>>,
        expression_gen: &ExpressionGen,
        code_gen: &CodeGenerator, 
    ) -> Result<String> {
        let mut result = String::new();

        let condition_str = match condition {
            Expression::Parenthesized(_) => {
                expression_gen.generate(condition, None)?
            }
            _ => {
                let expr_str = expression_gen.generate(condition, None)?;
                format!("({})", expr_str)
            }
        };

        result.push_str(&format!("if {} {{\n", condition_str));
        result.push_str(&self.block_gen.generate(then_block, expression_gen, code_gen)?);
        result.push_str("}");

        if let Some(else_clause) = else_clause {
            result.push_str(&self.else_clause_gen(
                else_clause,
                expression_gen,
                code_gen
            )?);
        }

        Ok(result)
    }

    fn else_clause_gen(
        &self,
        else_clause: &Box<ElseClause>,
        expression_gen: &ExpressionGen,
        code_gen: &CodeGenerator, 
    ) -> Result<String> {
        let mut result = String::new();

        result.push_str(" else");
        match else_clause.as_ref() {
            ElseClause::ElseIf(ast_node) => {
                match ast_node.as_ref() {
                    AstNode::IfStatement { condition, then_block, else_clause } => {
                        let raw = self.generate(
                            condition,
                            then_block,
                            else_clause,
                            expression_gen,
                            code_gen
                        )?;
                        result.push_str(&raw);

                        Ok(result)
                    }
                    _ => {
                        Err(anyhow::anyhow!("Invalid ElseClause type"))
                    }
                }
            } 
            ElseClause::Else (body) => {
                result.push_str(" {\n");
                result.push_str(&self.block_gen.generate(body, expression_gen, code_gen)?);
                result.push_str("}");

                Ok(result)
            } 
        }
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{codegen::CodeGenerator, language::parser::CorrozyParserImpl, utils::test_utils::default_corrozy_config};


    #[test]
    fn test_generate_simple_if() {
        let mut parser = CorrozyParserImpl::new();

        let ast = parser.parse(r#"
            if (true) {
                println("ok");
            }
        "#).unwrap();

        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();

        assert!(php.contains("if (true) {"));
        assert!(php.contains(r#"echo "ok" . "\n""#));
    }

    #[test]
    fn test_generate_if_else() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse(r#"
            if (true) {
                println("ok");
            } else {
                println("not ok");
            }
        "#).unwrap();
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();
        assert!(php.contains("if (true) {"));
        assert!(php.contains(r#"echo "ok" . "\n""#));
        assert!(php.contains("} else {"));
        assert!(php.contains(r#"echo "not ok" . "\n""#));
    }

    #[test]
    fn test_generate_if_elseif_else() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse(r#"
            if (x > 0) {
                println("positive");
            } else if (x < 0) {
                println("negative");
            } else {
                println("zero");
            }
        "#).unwrap();
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();
        assert!(php.contains("if ($x > 0) {"));
        assert!(php.contains(r#"echo "positive" . "\n""#));
        assert!(php.contains("} elseif ($x < 0) {"));
        assert!(php.contains(r#"echo "negative" . "\n""#));
        assert!(php.contains("} else {"));
        assert!(php.contains(r#"echo "zero" . "\n""#));
    }
    
    #[test]
    fn test_generate_if_elseif() {
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse(r#"
            if (x > 0) {
                println("positive");
            } else if (x < 0) {
                println("negative");
            }
        "#).unwrap();
        let code_gen = CodeGenerator::new(Rc::new(default_corrozy_config()));
        let php = code_gen.generate(&ast).unwrap();
        assert!(php.contains("if ($x > 0) {"));
        assert!(php.contains(r#"echo "positive" . "\n""#));
        assert!(php.contains("} elseif ($x < 0) {"));
        assert!(php.contains(r#"echo "negative" . "\n""#));
    }
}