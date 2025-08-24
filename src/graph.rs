use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
/// Generates a call graph from the analyzed code files.
use std::process::Command;

use crate::file::CodeFile;

const RESULT_DOT_PATH: &str = "./graph.dot";
const RESULT_PNG_PATH: &str = "./graph.png";

/// Writes the call graph in DOT format to the specified file.
///
/// # Arguments
/// * `files` - Analyzed code files.
/// * `path` - The file path where the DOT representation will be written.
///
pub fn write_dot(files: &Vec<CodeFile>, path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    writeln!(&mut file, "digraph CallGraph {{")?;
    // Set graph attributes
    writeln!(&mut file, "graph [ranksep=1.5, nodesep=1.0];")?;

    // Build lookup: function name -> fully qualified name
    let mut func_map = HashMap::new();
    for code_file in files {
        for func in &code_file.get_functions() {
            let fq_name = format!("{}::{}", code_file.get_name(), func.get_name());
            func_map.insert(func.get_name().clone(), fq_name);
        }
    }

    // Create clusters
    for (i, code_file) in files.iter().enumerate() {
        writeln!(&mut file, "  subgraph cluster_{i} {{")?;
        writeln!(&mut file, "    label = \"{}\";", code_file.get_name())?;
        writeln!(&mut file, "    style = filled;")?;
        writeln!(&mut file, "    color = lightgrey;")?;

        for func in &code_file.get_functions() {
            let fq_name = format!("{}::{}", code_file.get_name(), func.get_name());
            writeln!(&mut file, "    \"{fq_name}\";")?;
        }

        writeln!(&mut file, "  }}")?;
    }

    // Draw edges using resolved names
    for code_file in files {
        for func in &code_file.get_functions() {
            let caller = format!("{}::{}", code_file.get_name(), func.get_name());
            for callee in &func.get_calls() {
                if let Some(fq_callee) = func_map.get(callee) {
                    writeln!(&mut file, "  \"{caller}\" -> \"{fq_callee}\";")?;
                } else {
                    // fallback: if function not found, still draw plain node
                    writeln!(&mut file, "  \"{caller}\" -> \"{callee}\";")?;
                }
            }
        }
    }

    writeln!(&mut file, "}}")?;
    Ok(())
}

/// Generates a PNG image from the DOT file.
///
/// # Arguments
/// * `dot_path` - The file path to the input DOT file.
/// * `output_path` - The file path to the output PNG image.
///
fn generate_png_from_dot(dot_path: &str, output_path: &str) -> std::io::Result<()> {
    let status = Command::new("dot")
        .arg("-Tpng")
        .arg(dot_path)
        .arg("-o")
        .arg(output_path)
        .status()?;

    if !status.success() {
        eprintln!("dot command failed with status: {}", status);
    }

    Ok(())
}

/// Generates a call graph from the analyzed code files
///
/// # Arguments
/// * `calls` - Analyzed code files.
///
pub fn generate_graph(calls: &Vec<CodeFile>) {
    match write_dot(calls, RESULT_DOT_PATH) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Couldn't create the graph: {}", e);
        }
    };

    match generate_png_from_dot(RESULT_DOT_PATH, RESULT_PNG_PATH) {
        Ok(_) => {
            println!("Generated PNG graph at {}", RESULT_PNG_PATH);
        }
        Err(e) => {
            eprintln!("Couldn't create the image: {}", e);
        }
    }
}
