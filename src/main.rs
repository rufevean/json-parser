use std::path::Path;
//use std::fs;
mod utils;
// use utils::extracting_vectors;
use utils::file_to_lexer;



fn main() {
    /*
    let type_vectors = extracting_vectors::extracting_type(&contents);
    let name_vectors = extracting_vectors::extracting_name(&contents);
    println!("{:?}", name_vectors);
    println!("{:?}", type_vectors);
    */

    let path = Path::new("input_test/test2.json");
    file_to_lexer::file_to_lexer(path);



}

/*
fn read_file() -> Result<String, std::io::Error> {
    let contents = fs::read_to_string("symbol_table.txt")?;
    Ok(contents)
}
*/
