use failure::Fail;

#[derive(Debug, Fail)]
pub enum TransactionError {
    #[fail(display="Could not load file: {}", 0)]
    LoadError(std::io::Error),
    #[fail(display="Could not parse file: {}", 0)]
                                   // Fail::Error -> TransactionError -> std::error::Error
    ParseError(serde_json::Error), // Error:         ParseError(         Error("expected `,` or `}`", line: 5, column: 24))
    #[fail(display="Error message: {}", 0)]
    Message(&'static str)
}

impl From<std::io::Error> for TransactionError {
    fn from(err: std::io::Error) -> TransactionError {
        TransactionError::LoadError(err)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(err: serde_json::Error) -> TransactionError {
        TransactionError::ParseError(err)
    }
}

impl From<&'static str> for TransactionError {
    fn from(err: &'static str) -> TransactionError {
        TransactionError::Message(err)
    }
}
