#[derive(Debug)]
pub enum MyErrors {
    FileReadError(String),
    MissingArgsError,
    MissingArgFilesError,
    MissingArgPatternError,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub pattern: String,
    pub files: Vec<String>,
    quiet: bool,
    ignore_case: bool,
}

impl Config {
    pub fn new(pattern: String, files: Vec<String>) -> Config {
        Config {
            pattern,
            files,
            quiet: false,
            ignore_case: false,
        }
    }

    // Function are defined by name, not by name+parameters
    // fn new(pattern: String, files: Vec<String>, quiet: bool) -> Config {
    //     Config {
    //         pattern,
    //         files,
    //         quiet,
    //     }
    // }

    // Therefore we will use the builder pattern !
    pub fn with_quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    pub fn is_quiet(&self) -> bool {
        self.quiet
    }

    pub fn with_case_ignored(mut self, ignore_case: bool) -> Self {
        self.ignore_case = ignore_case;
        self
    }

    pub fn is_case_ignored(&self) -> bool {
        self.ignore_case
    }
}

// impl From<std::io::Error> for MyErrors {
//     fn from(e: std::io::Error) -> Self {
//         dbg!(&e);
//         MyErrors::FileReadError(e.to_string())
//     }
// }
