use std::env;
use std::path::Path;

use crate::file::analyse_files;
use crate::function::analyse_functions;
use crate::graph::generate_graph;

mod file;
mod function;
mod graph;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: ./{} <folder path>", args[0]);
    }

    // Path of the directory to analyze
    let path = Path::new(&args[1]);

    // Analyze the files in the given directory
    let mut files = analyse_files(path);

    // Analyze the functions in the files
    analyse_functions(&mut files);

    // Generate the call graph
    generate_graph(&files);
}
