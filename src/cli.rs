use aegrep::types::{Config, MyErrors};

pub fn parse_args(mut args: Vec<String>) -> Result<Config, MyErrors> {
    args.reverse();

    let _name = args.pop().ok_or(MyErrors::MissingArgsError)?;
    let query = args.pop().ok_or(MyErrors::MissingArgPatternError)?;
    if args.is_empty() {
        return Err(MyErrors::MissingArgFilesError);
    }

    Ok(Config::new(query, args))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn missing_name() {
        let args = vec![];
        assert!(parse_args(args).is_err());
    }

    #[test]
    fn missing_query() {
        let args = vec![String::from("name")];
        assert!(parse_args(args).is_err());
    }

    #[test]
    fn missing_first_file() {
        let args = vec![String::from("name"), String::from("query")];
        assert!(parse_args(args).is_err());
    }

    #[test]
    fn ok_arguments() {
        let args = vec![
            String::from("name"),
            String::from("query"),
            String::from("file.txt"),
        ];
        assert!(parse_args(args).is_ok());
    }
}
