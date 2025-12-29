mod pretty;
pub mod types;

use crate::types::MyErrors;
use std::fs;

fn single_search(query: &String, file: &String, quiet: bool) -> Result<usize, MyErrors> {
    let mut matchs = 0;
    let lines = fs::read_to_string(file).map_err(|_| MyErrors::FileReadError(file.clone()))?;
    for (line_no, line) in lines.lines().enumerate() {
        matchs += 1;
        if !quiet && let Some(char_no) = line.find(query) {
            pretty::print_found(line, line_no, char_no, file, query);
        }
    }

    Ok(matchs)
}

pub fn search(query: String, files: Vec<String>, quiet: bool) -> Result<usize, MyErrors> {
    if files.is_empty() {
        return Err(MyErrors::MissingArgsError);
    }
    let mut files = files.clone();
    let mut matchs = 0;

    while let Some(file) = files.pop() {
        matchs += single_search(&query, &file, quiet)?;
    }

    Ok(matchs)
}
