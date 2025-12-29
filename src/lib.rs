pub mod pretty;
pub mod types;

use crate::types::MyErrors;
use std::fs;

fn single_search(query: &String, file: &String) -> Result<(), MyErrors> {
    let lines = fs::read_to_string(file).map_err(|_| MyErrors::FileReadError(file.clone()))?;
    for (line_no, line) in lines.lines().enumerate() {
        // match line.find(query) {
        //     Some(_) => {
        //         println!("found it at {}", line);
        //     }
        //     None => (),
        // }
        // if let Some(_) = line.find(query) {
        //     println!("found it at {}", line);
        // }
        // if line.contains(query) {
        //     print_found(line, line_no, 0, file, query);
        // }
        if let Some(char_no) = line.find(query) {
            pretty::print_found(line, line_no, char_no, file, query);
        }
    }
    Ok(())
}

pub fn search(mut args: Vec<String>) -> Result<(), MyErrors> {
    let query = args.pop().ok_or(MyErrors::MissingArgsError)?;
    // Force the at least one file
    let first_file = args.pop().ok_or(MyErrors::MissingArgsError)?;

    single_search(&query, &first_file)?;
    while let Some(file) = args.pop() {
        single_search(&query, &file)?;
    }

    Ok(())
}
