#[derive(Debug)]
pub enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
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
