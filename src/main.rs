use std::path::Path;
//use std::fs;
mod utils;
// use utils::extracting_vectors;
use utils::file_to_lexer;

fn main() {
    let path = Path::new("input_test/test2.json");
    file_to_lexer::file_to_lexer(path);
}
