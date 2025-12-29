mod pretty;
pub mod types;

use crate::types::MyErrors;
use std::fs;

fn single_search(query: &String, file: &String) -> Result<(), MyErrors> {
    let lines = fs::read_to_string(file).map_err(|_| MyErrors::FileReadError(file.clone()))?;
    for (line_no, line) in lines.lines().enumerate() {
        if let Some(char_no) = line.find(query) {
            pretty::print_found(line, line_no, char_no, file, query);
        }
    }
    Ok(())
}

pub fn search(query: String, files: Vec<String>) -> Result<(), MyErrors> {
    if files.is_empty() {
        return Err(MyErrors::MissingArgsError);
    }
    let mut files = files.clone();

    while let Some(file) = files.pop() {
        single_search(&query, &file)?;
    }

    Ok(())
}
