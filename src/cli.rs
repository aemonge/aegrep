use std::env;

use aegrep::types::{Config, MyErrors};

pub fn parse_args() -> Result<Config, MyErrors> {
    let mut args: Vec<String> = env::args().collect();
    args.reverse();

    let _name = args.pop().ok_or(MyErrors::MissingArgsError)?;
    let query = args.pop().ok_or(MyErrors::MissingArgPatternError)?;
    if args.is_empty() {
        return Err(MyErrors::MissingArgFilesError);
    }

    Ok(Config::new(query, args))
}
