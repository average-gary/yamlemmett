use std::fs;
use std::path::Path;
use clap::Parser;
use serde_json::Value;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input YAML file (if not provided, reads from stdin)
    #[arg(short, long)]
    input: Option<String>,

    /// Output JSON file (if not provided, writes to stdout)
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read input
    let yaml_content = if let Some(input_path) = args.input {
        fs::read_to_string(input_path)?
    } else {
        let mut content = String::new();
        io::stdin().read_to_string(&mut content)?;
        content
    };

    // Parse YAML and convert to JSON
    let value: Value = serde_yaml::from_str(&yaml_content)?;
    let json_string = serde_json::to_string_pretty(&value)?;

    // Write output
    if let Some(output_path) = args.output {
        fs::write(output_path, json_string)?;
    } else {
        println!("{}", json_string);
    }

    Ok(())
}
