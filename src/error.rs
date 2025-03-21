#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed load a .env file: '{line_content}' on line {line_number} is invalid")]
    DotenvyParse {
        line_content: String,
        line_number: usize,
    },
    #[error("failed to read a .env file: {0}")]
    DotenvyIo(std::io::Error),

    #[error("{0}")]
    Custom(String),
}
