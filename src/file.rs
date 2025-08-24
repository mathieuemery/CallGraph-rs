use anyhow::Result;
use quote::ToTokens;
use std::fs::read_dir;
use std::path::Path;
use syn::{Item, parse_file};

use crate::function::Function;

/// The file extension for Rust source files.
const FILE_EXTENSION: &str = "rs";

/// Represents a Rust source code file.
pub struct CodeFile {
    file_name: String,
    functions: Vec<Function>,
}

impl CodeFile {
    /// Creates a new code file.
    ///
    /// # Arguments
    /// * `file_name` - The name of the file.
    ///
    /// # Returns
    /// A new instance of `CodeFile`.
    ///
    fn new(file_name: String) -> Self {
        CodeFile {
            file_name,
            functions: Vec::new(),
        }
    }

    /// Adds a function to the code file.
    ///
    /// # Arguments
    /// * `function` - The function to add.
    ///
    fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    /// Gets the functions in the code file.
    ///
    /// # Returns
    /// A vector of functions in the code file.
    ///
    pub fn get_functions(&self) -> Vec<Function> {
        self.functions.clone()
    }

    /// Gets a mutable iterator over the functions in the code file.
    ///
    /// # Returns
    /// An iterator over mutable references to functions in the code file.
    ///
    pub fn get_functions_mut(&mut self) -> impl Iterator<Item = &mut Function> {
        self.functions.iter_mut()
    }

    /// Gets the name of the code file.
    ///
    /// # Returns
    /// The name of the code file.
    ///
    pub fn get_name(&self) -> String {
        self.file_name.clone()
    }
}

/// Collects all Rust source files in a directory recursively.
///
/// # Arguments
/// * `path` - The path to the directory to search.
///
/// # Returns
/// A vector of paths to the found Rust source files.
///
fn collect_files_recursively(path: &Path) -> Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == FILE_EXTENSION {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        } else if path.is_dir() {
            // recursively analysis found folders
            files.extend(collect_files_recursively(&path)?);
        }
    }

    Ok(files)
}
/// Extracts the file name from a file path.
///
/// # Arguments
/// * `file_path` - The path to the file.
///
/// # Returns
/// The file name extracted from the file path.
///
fn extract_filename(file_path: &str) -> String {
    Path::new(file_path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

/// Extracts the functions and their content from a Rust source file.
///
/// # Arguments
/// * `path` - The path to the file.
///
/// # Returns
/// A `CodeFile` instance containing the extracted functions and their content.
///
fn extract_functions_with_content(path: &String) -> Result<CodeFile> {
    let code = std::fs::read_to_string(path)?;
    let file_name = extract_filename(path);

    let syntax = parse_file(&code)?; // Parse full file

    let mut result = CodeFile::new(file_name);

    for item in syntax.items {
        if let Item::Fn(func) = item {
            let name = func.sig.ident.to_string();

            // Only take the function body
            let content = func.block.to_token_stream().to_string();

            let function = Function::new(name, content);

            result.add_function(function);
        }
    }

    Ok(result)
}

/// Analyzes Rust source files in a directory.
///
/// # Arguments
/// * `path` - The path to the directory to analyze.
///
/// # Returns
/// A vector of `CodeFile` instances containing the extracted functions and their content.
///
pub fn analyse_files(path: &Path) -> Vec<CodeFile> {
    let files_path = match collect_files_recursively(path) {
        Ok(f) => f,
        Err(e) => {
            panic!("Couldn't get the names: {}", e);
        }
    };

    let mut result = Vec::new();

    for file in &files_path {
        let code_file = match extract_functions_with_content(file) {
            Ok(c) => c,
            Err(e) => {
                panic!("Error while extracting blocks: {}", e);
            }
        };
        result.push(code_file);
    }

    result
}
