use crate::{Config, config::{NamespaceConfig, NamespaceMode, TranspilerConfig}};

pub fn default_corrozy_config() -> Config {
    Config {
        transpiler: TranspilerConfig {
            src_dir: "src".to_string(),
            output_dir: "out".to_string(),
            strict_types: true,
            include_comments: false,
        },
        namespace: NamespaceConfig {
            base_namespace: "MyApp".to_string(),
            separator: "\\".to_string(),
            mode: NamespaceMode::Auto,
        },
    }
}