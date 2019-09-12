use libconsensus::errors::Error as BaseError;
use std::option::NoneError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Base(BaseError),
    Bincode(bincode::Error),
    Sled(sled::Error),
    Io(std::io::Error),
    SerdeJson(serde_json::error::Error),
    NoneError(NoneError),
}

impl From<NoneError> for Error {
    #[inline]
    fn from(none_error: NoneError) -> Error {
        Error::NoneError(none_error)
    }
}

impl From<sled::Error> for Error {
    #[inline]
    fn from(sled_error: sled::Error) -> Error {
        Error::Sled(sled_error)
    }
}

impl From<bincode::Error> for Error {
    #[inline]
    fn from(bincode_error: bincode::Error) -> Error {
        Error::Bincode(bincode_error)
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(io_error: std::io::Error) -> Error {
        Error::Io(io_error)
    }
}

impl From<serde_json::error::Error> for Error {
    #[inline]
    fn from(json_error: serde_json::error::Error) -> Error {
        Error::SerdeJson(json_error)
    }
}

impl From<BaseError> for Error {
    #[inline]
    fn from(b: BaseError) -> Error {
        Error::Base(b)
    }
}
