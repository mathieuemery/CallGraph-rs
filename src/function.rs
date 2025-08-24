use crate::file::CodeFile;

/// Represents a function in the codebase.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Function {
    name: String,
    body: String,
    calls: Vec<String>,
}

impl Function {
    /// Creates a new function.
    ///
    /// # Arguments
    /// * `name` - The name of the function.
    /// * `body` - The body of the function.
    ///
    /// # Returns
    /// A new instance of `Function`.
    ///
    pub fn new(name: String, body: String) -> Self {
        Function {
            name,
            body,
            calls: Vec::new(),
        }
    }

    /// Gets the name of the function.
    ///
    /// # Returns
    /// The name of the function.
    ///
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Adds function calls to the function.
    ///
    /// # Arguments
    /// * `call` - A vector of function names that this function calls.
    ///
    fn add_calls(&mut self, mut call: Vec<String>) {
        self.calls.append(&mut call);
    }

    /// Gets the function calls made by this function.
    ///
    /// # Returns
    /// A vector of function names that this function calls.
    ///
    pub fn get_calls(&self) -> Vec<String> {
        self.calls.clone()
    }
}
/// Finds function calls in a given string.
///
/// # Arguments
/// * `haystack` - The string to search within.
/// * `needles` - A list of function names to search for.
///
/// # Returns
/// A vector of function names that were found.
///
fn find_function_calls(haystack: &String, needles: &Vec<String>) -> Vec<String> {
    needles
        .iter()
        .filter(|word| haystack.contains(word.as_str()))
        .cloned() // convert &String back to String
        .collect()
}

/// Gets the found methods from the analyzed code files.
///
/// # Arguments
/// * `files` - The analyzed code files.
///
/// # Returns
/// A vector of function names that were found.
///
fn get_found_methods(files: &Vec<CodeFile>) -> Vec<String> {
    let mut result = Vec::new();
    for file in files {
        let functions = file.get_functions();
        for func in functions {
            result.push(func.get_name());
        }
    }

    result
}

/// Analyzes the functions in the given code files.
///
/// # Arguments
/// * `files` - The code files to analyze.
///
pub fn analyse_functions(files: &mut Vec<CodeFile>) {
    // Get the full list of functions
    let function_list = get_found_methods(files);

    for file in files {
        for function in file.get_functions_mut() {
            let calls = find_function_calls(&function.body, &function_list);
            /*for call in calls {
                println!("Function {} calls {}() !", function.get_name(), call);
            }*/
            function.add_calls(calls);
        }
    }
}
