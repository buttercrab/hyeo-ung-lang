use std::fmt;
use std::fmt::{Debug, Display};
use std::io::ErrorKind;
use std::string::FromUtf8Error;

#[derive(Clone)]
pub struct Error {
    msg: String,
    note: String,
}

impl Error {
    pub fn new(msg: String, note: String) -> Error {
        Error { msg, note }
    }

    pub fn from<T>(err: T, note: String) -> Error
    where
        T: Debug,
    {
        Error::new(format!("{:?}", err), note)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.msg)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::from(e, String::new())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Self::from(e, String::new())
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        Self::new(ErrorKind::Other, e)
    }
}
