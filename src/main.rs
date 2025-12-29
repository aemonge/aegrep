mod cli;
use aegrep::types::MyErrors;

fn main() {
    let _ = match cli::parse_args() {
        Ok((query, files)) => aegrep::search(query, files),
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
