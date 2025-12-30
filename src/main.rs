mod cli;
use aegrep::types::MyErrors;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _ = match cli::parse_args(args) {
        Ok(config) => aegrep::search(
            config.pattern.clone(),
            config.files.clone(),
            config.is_quiet(),
        ),
        // Ok(config) => aegrep::search(config.pattern, config.files, config.is_quiet()),
        Err(MyErrors::MissingArgPatternError) => {
            panic!("Missing pattern to search for")
        }
        Err(MyErrors::MissingArgFilesError) => {
            panic!("Missing file to search withing")
        }
        Err(MyErrors::MissingArgsError) => {
            panic!("Missing arguments")
        }
        Err(MyErrors::FileReadError(e)) => {
            panic!("Error reading file: {}", e)
        }
    };
}
