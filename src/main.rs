use owo_colors::OwoColorize; // Powers Up Strings with colors
use std::{env, fs};

#[derive(Debug)]
enum MyErrors {
    FileReadError(String),
    MissingArgsError,
}

fn print_found(line: &str, line_no: usize, char_no: usize, file: &str, query: &str) {
    let colored_line = line.replace(query, &query.bright_yellow().to_string());
    println!(
        "{}:{}:{} {}",
        file.bold(),
        (line_no + 1).to_string().bold(),
        (char_no).to_string().bold(),
        colored_line
    );
}

// impl From<std::io::Error> for MyErrors {
//     fn from(e: std::io::Error) -> Self {
//         dbg!(&e);
//         MyErrors::FileReadError(e.to_string())
//     }
// }

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
            print_found(line, line_no, char_no, file, query);
        }
    }
    Ok(())
}

fn search(mut args: Vec<String>) -> Result<(), MyErrors> {
    let query = args.pop().ok_or(MyErrors::MissingArgsError)?;
    // Force the at least one file
    let first_file = args.pop().ok_or(MyErrors::MissingArgsError)?;

    single_search(&query, &first_file)?;
    while let Some(file) = args.pop() {
        single_search(&query, &file)?;
    }

    Ok(())
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.reverse();
    let _name = args.pop().expect("Should at least contain the script name");

    match search(args) {
        Ok(_) => (),
        Err(MyErrors::MissingArgsError) => {
            panic!("Missing arguments")
        }
        Err(MyErrors::FileReadError(f)) => {
            panic!("Error while reading: {}", f)
        }
    }
}
