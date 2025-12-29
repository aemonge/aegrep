use aegrep::types::MyErrors;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.reverse();
    let _name = args.pop().expect("Should at least contain the script name");

    match aegrep::search(args) {
        Ok(_) => (),
        Err(MyErrors::MissingArgsError) => {
            panic!("Missing arguments")
        }
        Err(MyErrors::FileReadError(f)) => {
            panic!("Error while reading: {}", f)
        }
    }
}
