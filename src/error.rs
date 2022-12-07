use std::{string::FromUtf16Error, array::TryFromSliceError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error: {0}")]
    String(String),

    #[error("Error: {0}")]
    FromUtf16Error(FromUtf16Error),

    #[error("Error: {0}")]
    TryFromSliceError(TryFromSliceError),
    
    #[error("Error while trying to offset {0} byte(s) from position {1} in buffer length of {2}")]
    TryOffsetError(usize,usize,usize),
    
    #[error("Error while trying to store {0} value at position {1} in buffer length of {2}")]
    TryStoreError(&'static str,usize,usize),
    
    #[error("Error while trying to store slice of {0} byte(s) at position {1} in buffer length of {2}")]
    TryStoreSliceError(usize,usize,usize),
    
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::String(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Error {
        Error::String(s.to_string())
    }
}

impl From<FromUtf16Error> for Error {
    fn from(e: FromUtf16Error) -> Error {
        Error::FromUtf16Error(e)
    }
}

impl From<TryFromSliceError> for Error {
    fn from(e: TryFromSliceError) -> Error {
        Error::TryFromSliceError(e)
    }
}