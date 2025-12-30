mod cli;
use aegrep::types::MyErrors;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    match cli::parse_args(args) {
        Ok(config) => {
            let _ = aegrep::search(
                config.pattern.clone(),
                config.files.clone(),
                config.is_quiet(),
                config.is_case_ignored(),
            );
            process::exit(0);
        }
        // Ok(config) => aegrep::search(config.pattern, config.files, config.is_quiet()),
        Err(MyErrors::MissingArgPatternError) => {
            eprintln!("Missing pattern to search for");
            process::exit(1);
        }
        Err(MyErrors::MissingArgFilesError) => {
            eprintln!("Missing file to search withing");
            process::exit(2);
        }
        Err(MyErrors::MissingArgsError) => {
            eprintln!("Missing arguments");
            process::exit(3);
        }
        Err(MyErrors::FileReadError(e)) => {
            eprintln!("Error reading file: {}", e);
            process::exit(4);
        }
    };
}
