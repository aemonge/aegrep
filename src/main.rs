mod cli;
use aegrep::types::MyErrors;

fn main() {
    let _ = match cli::parse_args() {
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
