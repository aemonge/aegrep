mod pretty;
pub mod types;

use crate::types::MyErrors;
use std::fs;

fn search_on_lines(query: &str, file_name: &str, lines: &str) -> Vec<String> {
    let mut found: Vec<String> = vec![];
    for (line_no, line) in lines.lines().enumerate() {
        if let Some(char_no) = line.find(query) {
            found.push(pretty::print_found(
                line, line_no, char_no, file_name, query,
            ));
        }
    }
    found
}

fn single_search(query: &str, file_name: &String) -> Result<Vec<String>, MyErrors> {
    let lines =
        fs::read_to_string(file_name).map_err(|_| MyErrors::FileReadError(file_name.clone()))?;

    Ok(search_on_lines(query, file_name, &lines))
}

pub fn search(query: String, files: Vec<String>, quiet: bool) -> Result<usize, MyErrors> {
    if files.is_empty() {
        return Err(MyErrors::MissingArgsError);
    }
    let mut files = files.clone();
    let mut matchs: Vec<String> = vec![];

    while let Some(file) = files.pop() {
        let e = single_search(&query, &file)?;
        matchs = [matchs, e].concat();
    }

    let found_n = matchs.len();

    if !quiet {
        for found in matchs {
            println!("{}", found);
        }
    }

    Ok(found_n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result_on_search_on_lines() {
        let query = "safe";
        let file_name = "test.tmp";
        let contents = "\
Rust
safe, fast, productive.
Pick three.";
        let result = search_on_lines(query, file_name, contents);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn no_result_on_search_on_lines() {
        let query = "duck";
        let file_name = "test.tmp";
        let contents = "\
Rust
safe, fast, productive.
Pick three.";
        let result = search_on_lines(query, file_name, contents);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn three_result_on_search_on_lines() {
        let query = "crab";
        let file_name = "test.tmp";
        let contents = "\
Rust, the crab
safe, fast, productive as a crab.
Pick three from crab.";
        let result = search_on_lines(query, file_name, contents);
        assert_eq!(result.len(), 3);
    }
}
