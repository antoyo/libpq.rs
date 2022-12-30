use std::fmt::Display;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    ParseIntError(std::num::ParseIntError),
    NulError(std::ffi::NulError),
    Backend(String),
    Unknow,
    Utf8(std::str::Utf8Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseIntError(ref error) => error.fmt(f),
            Error::NulError(ref error) => error.fmt(f),
            Error::Backend(ref error) => error.fmt(f),
            Error::Unknow => "Unknown error".fmt(f),
            Error::Utf8(ref error) => error.fmt(f),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error::ParseIntError(error)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(error: std::ffi::NulError) -> Self {
        Error::NulError(error)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::Utf8(error)
    }
}
