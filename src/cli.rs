use std::env;

use aegrep::types::{Config, MyErrors};

pub fn parse_args(mut args: Vec<String>) -> Result<Config, MyErrors> {
    args.reverse();

    let _name = args.pop().ok_or(MyErrors::MissingArgsError)?;
    let query = args.pop().ok_or(MyErrors::MissingArgPatternError)?;
    if args.is_empty() {
        return Err(MyErrors::MissingArgFilesError);
    }
    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Ok(Config::new(query, args).with_case_ignored(ignore_case))
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

    #[test]
    fn ok_arguments_no_env() {
        let args = vec![
            String::from("name"),
            String::from("query"),
            String::from("file.txt"),
        ];
        let result = parse_args(args);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_case_ignored());
    }

    #[test]
    fn ok_arguments_env_ignore_case() {
        unsafe {
            env::set_var("IGNORE_CASE", "1");
            let args = vec![
                String::from("name"),
                String::from("query"),
                String::from("file.txt"),
            ];
            let result = parse_args(args);
            let result = result.unwrap();
            assert!(result.is_case_ignored());
        }
    }
}
