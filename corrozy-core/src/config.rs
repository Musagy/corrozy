use std::path::Path;
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    // pub project: ProjectConfig,
    pub transpiler: TranspilerConfig,
    pub namespace: NamespaceConfig,
    // pub syntax: SyntaxConfig,
    // pub debugging: DebuggingConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TranspilerConfig {
    pub output_dir: String,
    pub src_dir: String,
    // pub namespace: String,
    pub strict_types: bool,
    pub include_comments: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NamespaceConfig {
    pub mode: NamespaceMode,
    pub separator: String,
    pub base_namespace: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NamespaceMode {
    Auto,
    Manual, 
    None,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }
}