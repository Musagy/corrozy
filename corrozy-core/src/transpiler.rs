use std::{path::Path, rc::Rc};

use anyhow::{anyhow, Ok, Result};
use walkdir::WalkDir;

use crate::{codegen::CodeGenerator, config::{Config, NamespaceMode}, parser::{ast::AstNode, parser::CorrozyParserImpl}};

pub struct Transpiler {
    config: Config,
}

impl Transpiler {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn transpile_project(
        &mut self,
        project_path: &Path
    ) -> Result<()> {
        let output_dir = project_path.join(&self.config.transpiler.output_dir);
        std::fs::create_dir_all(&output_dir)?;
        
        for entry in WalkDir::new(project_path) {
          let entry = entry?;
          if let Some(ext) = entry.path().extension() {
            if ext == "crz" {
                self.transpile_file(entry.path(), &output_dir, &project_path)?;
            }
          }
        }
        
        Ok(())
    }

    /// Transpile a single file from Corrozy to PHP
    fn transpile_file(
        &self,
        input_path: &Path,
        output_dir: &Path,
        project_path: &Path
    ) -> Result<()> {
        let content = std::fs::read_to_string(input_path)?;
        
        let mut parser = CorrozyParserImpl::new();
        let ast = parser.parse(&content)?;

        let relative_path = input_path
            .strip_prefix(project_path)
            .map_err(|_| anyhow!("Input path is not within project path"))?;
        
        let src_dir_path = Path::new(&self.config.transpiler.src_dir);
        let src_dir_clean = if src_dir_path.starts_with("./") {
            src_dir_path.strip_prefix("./").unwrap()
        } else {
            src_dir_path
        };
        
        let output_relative_path = if relative_path.starts_with(src_dir_clean) {
            relative_path.strip_prefix(src_dir_clean)
                .unwrap_or(relative_path)
        } else {
            relative_path
        };

        let php_code = self.generate_php(output_relative_path, &ast)?;
        
        let output_file_path = output_dir.join(output_relative_path).with_extension("php");
        
        if let Some(parent_dir) = output_file_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }

        std::fs::write(output_file_path, php_code)?;

        Ok(())
    }

    fn generate_php(
        &self,
        relative_path: &Path,
        ast: &[AstNode],
    ) -> Result<String> {
        let mut output = String::new();

        output.push_str("<?php\n");

        if self.config.transpiler.strict_types {
            output.push_str("declare(strict_types=1);\n\n");
        }
        
        if !self.config.namespace.base_namespace.is_empty() {
            let namespace = self.generate_namespace(relative_path);
            if let Some(ns) = namespace {
                output.push_str(&format!("namespace {};\n\n", ns));
            }
        }

        let code_gen = CodeGenerator::new(Rc::new(self.config.clone()));
        let generated_code = code_gen.generate(ast)?;
        output.push_str(&generated_code);

        Ok(output)
    }

    fn generate_namespace(&self, relative_path: &Path) -> Option<String> {
        match self.config.namespace.mode {
            NamespaceMode::None => None,
            
            NamespaceMode::Manual => {
                if self.config.namespace.base_namespace.is_empty() {
                    None
                } else {
                    // TODO: Make fn to get namespace from file in first line
                    Some(self.convert_separator(&self.config.namespace.base_namespace))
                }
            },
            
            NamespaceMode::Auto => {
                self.generate_auto_namespace(relative_path)
            }
        }
    }
    
    fn generate_auto_namespace(&self, relative_path: &Path) -> Option<String> {
        let dir_path = relative_path.parent()?;
        
        if dir_path.as_os_str().is_empty() {
            return Some(self.convert_separator(&self.config.namespace.base_namespace));
        }
        
        let mut namespace_parts = vec![self.config.namespace.base_namespace.clone()];
        
        for component in dir_path.components() {
            let dir_name = component.as_os_str().to_str()?;
            namespace_parts.push(self.pascal_case(dir_name));
        }
        
        let namespace = namespace_parts.join(&self.config.namespace.separator);
        Some(self.convert_separator(&namespace))
    }

    fn convert_separator(&self, namespace: &str) -> String {
        let separator = &self.config.namespace.separator;
        if !(separator == "\\") {
            namespace.replace(separator, "\\")
        } else {
            namespace.to_string()
        }
    }
    
    fn pascal_case(&self, s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect()
    }
}
