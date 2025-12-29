use std::env;

use aegrep::types::MyErrors;

pub fn parse_args() -> Result<(String, Vec<String>), aegrep::types::MyErrors> {
    let mut args: Vec<String> = env::args().collect();
    args.reverse();

    let _name = args.pop().ok_or(MyErrors::MissingArgsError)?;
    let query = args.pop().ok_or(MyErrors::MissingArgPatternError)?;
    if args.is_empty() {
        return Err(MyErrors::MissingArgFilesError);
    }

    Ok((query, args))
}
