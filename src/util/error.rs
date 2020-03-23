use std::fmt;
use std::fmt::{Debug, Display};
use std::io::ErrorKind;
use std::string::FromUtf8Error;

#[derive(Clone, Debug)]
pub struct Error {
    msg: String,
    note: String,
}

impl Error {
    pub fn new<A, B>(msg: A, note: B) -> Error
    where
        A: Into<String>,
        B: Into<String>,
    {
        Error {
            msg: msg.into(),
            note: note.into(),
        }
    }

    pub fn from<T, S>(err: T, note: S) -> Error
    where
        T: Debug,
        S: Into<String>,
    {
        Error::new(format!("{:?}", err), note)
    }

    pub fn get_msg(&self) -> String {
        self.msg.clone()
    }

    pub fn get_note(&self) -> String {
        self.note.clone()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::from(e, "")
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Self::from(e, "")
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        Self::new(ErrorKind::Other, e)
    }
}

pub fn add_note<T, E, S>(result: Result<T, E>, note: S) -> Result<T, Error>
where
    E: Debug,
    S: Into<String>,
{
    result.map_err(|e| Error::from(e, note))
}
