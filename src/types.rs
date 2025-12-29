#[derive(Debug)]
pub enum MyErrors {
    FileReadError(String),
    MissingArgsError,
    MissingArgFilesError,
    MissingArgPatternError,
}

// impl From<std::io::Error> for MyErrors {
//     fn from(e: std::io::Error) -> Self {
//         dbg!(&e);
//         MyErrors::FileReadError(e.to_string())
//     }
// }
