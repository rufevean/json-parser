
use std::fs;
mod utils;
use utils::extracting_vectors;

fn main() {
    let contents = read_file().expect("Something went wrong reading the file");
    let type_vectors = extracting_vectors::extracting_type(&contents);
    let name_vectors = extracting_vectors::extracting_name(&contents);
    println!("{:?}", name_vectors);
    println!("{:?}", type_vectors);
}


fn read_file() -> Result<String, std::io::Error> {
    let contents = fs::read_to_string("symbol_table.txt")?;
    Ok(contents)
}

