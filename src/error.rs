use std::io;
use std::io::Error as IOError;
use thiserror::Error;
pub type FstResult<T> = Result<T, FstError>;

#[derive(Error, Debug)]
pub enum FstError {
    #[error("Unexpected: {0}, {1}")]
    UnexpectIO(String, io::Error),
    #[error("Unexpected: {0}")]
    Unexpected(String),
    #[error("reader fst EOF")]
    Eof,
    #[error("Fail")]
    Fail,
    #[error("NotFound")]
    NotFound,
    #[error("Greater")]
    Greater,
    #[error("encode fail from :{0}")]
    EncodeFail(String),
    #[error("io write fail from :{0}")]
    IoWriteFail(io::Error),
}

impl From<&str> for FstError {
    fn from(e: &str) -> Self {
        FstError::Unexpected(e.to_string())
    }
}

impl From<(&str, io::Error)> for FstError {
    fn from(e: (&str, io::Error)) -> Self {
        FstError::UnexpectIO(e.0.to_string(), e.1)
    }
}

impl From<String> for FstError {
    fn from(e: String) -> Self {
        FstError::Unexpected(e)
    }
}

impl From<IOError> for FstError {
    fn from(e: IOError) -> Self {
        FstError::Unexpected(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::io::ErrorKind;

    #[derive(Error, Debug)]
    pub enum DataStoreError {
        #[error("data store disconnected:{0}")]
        Disconnect(#[from] io::Error),
        #[error("the data for key `{0}` is not available")]
        Redaction(String),
        #[error("invalid header (expected {expected:?}, found {found:?})")]
        InvalidHeader { expected: String, found: String },
        #[error("unknown data store error")]
        Unknown,
    }

    fn get_error1() -> DataStoreError {
        DataStoreError::Redaction("xxxx".to_string())
    }

    fn get_error2() -> DataStoreError {
        DataStoreError::InvalidHeader {
            expected: "a".to_string(),
            found: "b".to_string(),
        }
    }

    fn get_error3() -> DataStoreError {
        DataStoreError::Disconnect(io::Error::from(ErrorKind::AddrInUse))
    }

    #[test]
    fn test_get_error() {
        println!("{}", get_error1());
        println!("{}", get_error2());
        println!("{}", get_error3());
    }
}
