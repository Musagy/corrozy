use anyhow::{anyhow, Result};

use crate::parser::{ast::{AstNode, Expression, PostfixSuffix}, parser::{CorrozyParserImpl, Rule}};

impl CorrozyParserImpl {
    pub fn parse_postfix_expression(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
        let mut inner_pairs = pair.into_inner();
    
        // 1. Inicializar la base (primary_expression)
        // El primer par SIEMPRE es el primary_expression.
        let base_pair = inner_pairs.next().unwrap();
        let mut current_expression = self.parse_primary_expression(base_pair)?;
        
        // 2. Iterar sobre todos los sufijos opcionales
        // Cada sufijo envolverá la `current_expression` anterior.
        for suffix_pair in inner_pairs {
            // En este punto, `suffix_pair` es uno de los grupos de sufijos:
            // Por ejemplo, para `a.b.c`:
            // - 1ra iteración: `suffix_pair` podría ser el grupo `(.identifier)` para `.b`
            // - 2da iteración: `suffix_pair` podría ser el grupo `(.identifier)` para `.c`
            
            let suffix_rule = suffix_pair.as_rule();
            let mut suffix_content = suffix_pair.into_inner();

            let new_suffix = match suffix_rule {
                // Indexación: `[expression]`
                Rule::expression => {
                    // Obtener el contenido del corchete (la Rule::expression)
                    let index_exp_pair = suffix_content.next().unwrap();
                    let index_expression = self.parse_expression(index_exp_pair)?;
                    
                    PostfixSuffix::Index {
                        index_expression: Box::new(index_expression)
                    }
                }
                
                // Acceso a propiedad: `.identifier`
                Rule::identifier => { 
                    // Obtener el contenido del punto (la Rule::identifier)
                    let ident_pair = suffix_content.next().unwrap();
                    let name = ident_pair.as_str().to_string();
                    
                    PostfixSuffix::Property { name }
                }
                
                // Llamada a método: `.function_call`
                Rule::function_call => {
                    // Obtener el contenido del punto (la Rule::function_call)
                    let call_pair = suffix_content.next().unwrap();
                    
                    let function_call_exp = self.parse_fn_call(call_pair)?;
                    
                    PostfixSuffix::MethodCall(function_call_exp) 
                }
                
                // Si no tienes reglas con nombre (`~ > Rule::...`) en Pest, 
                // este `_` catchall podría ser necesario, pero es mejor usar reglas con nombre.
                _ => return Err(anyhow!("Unexpected postfix operation: {:?}", suffix_rule)),
            };

            // 3. ENVOLVER: El resultado de la iteración anterior (current_expression)
            // se convierte en la base del nuevo Expression::Postfix.
            current_expression = Expression::Postfix {
                base: Box::new(current_expression),
                suffix: Some(new_suffix),
            };
        }

        Ok(current_expression)
    }
    
    fn parse_primary_expression(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<Expression> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::literal => {
                    return Ok(Expression::Literal(self.parse_literal(inner_pair)?));
                }
                Rule::identifier => {
                    return Ok(Expression::Variable(inner_pair.as_str().to_string()));
                }
                Rule::function_call => {
                    return Ok(Expression::FunctionCall(self.parse_fn_call(inner_pair)?));
                }
                Rule::expression => {
                    return Ok(Expression::Parenthesized(Box::new(self.parse_expression(inner_pair)?)));
                }
                _ => {}
            }
        }
        Err(anyhow!("Unknown primary expression"))
    }

    pub fn parse_define_type(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<String> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::type_annotation => {
                    return self.parse_type_annotation(inner_pair);
                }
                _ => {}
            }
        }
        Err(anyhow!("No type annotation found")) // o tu tipo de error
    }

    fn parse_type_annotation(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<String> {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::basic_type | Rule::custom_type => {
                    return Ok(inner_pair.as_str().to_string());
                }
                _ => {}
            }
        }
        Err(anyhow!("Invalid type annotation"))
    }

    pub fn parse_declaration_declaration(&mut self, pair: pest::iterators::Pair<Rule>) -> Result<AstNode> {
        let is_constant = pair.as_rule() == Rule::constant_declaration;
        let mut var_type: Option<String> = None;
        let mut name = String::new();
        let mut value = None;
        
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::define_type => {
                    var_type = Some(self.parse_define_type(inner_pair)?);
                }
                Rule::identifier => {
                    name = inner_pair.as_str().to_string();
                }
                Rule::expression => {
                    value = Some(Box::new(self.parse_expression(inner_pair)?));
                }
                _ => {}
            }
        }
        
        let val = value.ok_or_else(|| anyhow!("Declaration missing value"))?;
        
        if is_constant {
            Ok(AstNode::ConstantDeclaration { name, const_type: var_type, value: val })
        } else {
            Ok(AstNode::VariableDeclaration { var_type, name, value: val })
        }
    }
}

#[cfg(test)]
mod test {
    
}