use std::fmt;
use std::fmt::{Debug, Display};
use std::io::ErrorKind;
use std::string::FromUtf8Error;

/// Error structure of this program
#[derive(Clone, Debug)]
pub struct Error {
    msg: String,
    note: String,
}

impl Error {
    /// Makes new `Error`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::util::error::Error;
    ///
    /// let e = Error::new("hello", "world");
    /// assert_eq!("hello", e.get_msg());
    /// assert_eq!("world", e.get_note());
    /// ```
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

    /// Makes `Error` from other error
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::util::error::Error;
    ///
    /// let io_error = std::io::Error::new(std::io::ErrorKind::Other, "hello");
    /// let s = format!("{:?}", io_error);
    /// let error = Error::from(io_error, "world");
    ///
    /// assert_eq!(s, error.get_msg());
    /// assert_eq!("world", error.get_note());
    /// ```
    pub fn from<T, S>(err: T, note: S) -> Error
    where
        T: Debug,
        S: Into<String>,
    {
        Error::new(format!("{:?}", err), note)
    }

    /// Return msg
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::util::error::Error;
    ///
    /// let e = Error::new("hello", "world");
    /// assert_eq!("hello", e.get_msg());
    /// ```
    pub fn get_msg(&self) -> String {
        self.msg.clone()
    }

    /// Return note
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::util::error::Error;
    ///
    /// let e = Error::new("hello", "world");
    /// assert_eq!("world", e.get_note());
    /// ```
    pub fn get_note(&self) -> String {
        self.note.clone()
    }
}

impl Display for Error {
    /// Display impl for `Error`
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::util::error::Error;
    ///
    /// let e = Error::new("hello", "");
    ///
    /// assert_eq!("hello", format!("{}", e));
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    /// From impl for `Error`
    fn from(e: std::io::Error) -> Self {
        Self::from(e, "")
    }
}

impl From<std::string::FromUtf8Error> for Error {
    /// From impl for `Error`
    fn from(e: FromUtf8Error) -> Self {
        Self::from(e, "")
    }
}

impl From<Error> for std::io::Error {
    /// From impl for `std::io::Error`
    fn from(e: Error) -> Self {
        Self::new(ErrorKind::Other, e)
    }
}

/// Make new `Error` adding note
///
/// # Examples
///
/// ```
/// use hyeong::util::error;
///
/// let io_error = Result::<(), _>::Err(std::io::Error::new(std::io::ErrorKind::Other, "hello"));
/// let e = error::add_note(io_error, "world").err().unwrap();
///
/// assert_eq!("world", e.get_note());
/// ```
pub fn add_note<T, E, S>(result: Result<T, E>, note: S) -> Result<T, Error>
where
    E: Debug,
    S: Into<String>,
{
    result.map_err(|e| Error::from(e, note))
}
