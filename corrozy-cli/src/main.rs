use std::path::PathBuf;
use anyhow::{Ok, Result};
use clap::Parser;
use corrozy_core::{config, transpiler};

#[derive(Parser)]
#[command(name = "corrozy")]
#[command(about = "Transpiler of Corrozy to PHP")]
struct Cli {
    /// Directory of project
    #[arg(default_value = ".")]
    path: PathBuf,

    /// File of configuration
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Mode verbose
    #[arg(short, long, action = clap::ArgAction::SetTrue)] 
    verbose: bool,
}

fn main () -> Result<()>{
    let cli = Cli::parse();

    let config_path = cli.path.join("corrozy.toml");
    let config = config::Config::load(&config_path)?;

    let mut transpiler = transpiler::Transpiler::new(config);

    transpiler.transpile_project(&cli.path)?;

    println!("Transpilation completed successfully!");
    
    Ok(())
}
