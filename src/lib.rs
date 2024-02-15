pub mod commands;
pub mod hyeong;

use std::fmt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use lazy_static::lazy_static;
use log::error;

lazy_static! {
    static ref HYEONG_DIR: PathBuf = dirs::home_dir().unwrap().join(".hyeong");
}

pub static ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static WARN_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Exits when error is thrown
///
/// It helps to throw multiple errors and exit at once.
pub fn error_barrier(msg: fmt::Arguments) {
    if ERROR_COUNT.load(Ordering::Acquire) > 0 {
        error!("{}", msg);
        std::process::exit(1);
    }
}

pub mod io {
    use std::io::{BufRead, Read, Write};

    /// Custom writer structure for redirecting output.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::Write;
    /// use hyeong::util::io::CustomWriter;
    ///
    /// let mut  a = CustomWriter::new(|_| Result::Ok(()));
    ///
    /// a.write_all("Hello, World!".as_bytes()).unwrap();
    /// assert_eq!("Hello, World!", a.to_string().unwrap());
    /// ```
    pub struct CustomWriter<T>
    where
        T: Fn(String) -> std::io::Result<()>,
    {
        buf: Vec<u8>,
        print_fn: T,
    }

    impl<T> Write for CustomWriter<T>
    where
        T: Fn(String) -> std::io::Result<()>,
    {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buf.append(&mut buf.to_vec());
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            let res = (self.print_fn)(self.to_string()?);
            self.buf = Vec::new();
            res
        }
    }

    impl<T> CustomWriter<T>
    where
        T: Fn(String) -> std::io::Result<()>,
    {
        /// Make new `CustomWriter`
        ///
        /// # Examples
        ///
        /// ```
        /// use std::io::Write;
        /// use hyeong::util::io::CustomWriter;
        ///
        /// let mut  a = CustomWriter::new(|_| Result::Ok(()));
        ///
        /// a.write_all("Hello, World!".as_bytes()).unwrap();
        /// assert_eq!("Hello, World!", a.to_string().unwrap());
        /// ```
        pub fn new(func: T) -> CustomWriter<T> {
            CustomWriter {
                buf: Vec::new(),
                print_fn: func,
            }
        }

        /// Return string that is written
        pub fn to_string(&self) -> std::io::Result<String> {
            String::from_utf8(self.buf.clone())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        }
    }

    /// Custom reader structure for other input source
    ///
    /// # Examples
    ///
    /// ```
    /// use hyeong::util::io::{CustomReader, ReadLine};
    ///
    /// let mut a = CustomReader::new(String::from("Hello, World!"));
    ///
    /// assert_eq!("Hello, World!", a.read_line_().unwrap());
    /// ```
    pub struct CustomReader {
        buf: String,
        idx: usize,
    }

    impl CustomReader {
        /// Make new `CustomReader`
        ///
        /// # Examples
        ///
        /// ```
        /// use hyeong::util::io::{CustomReader, ReadLine};
        ///
        /// let mut a = CustomReader::new(String::from("Hello, World!"));
        ///
        /// assert_eq!("Hello, World!", a.read_line_().unwrap());
        /// ```
        pub fn new(s: String) -> CustomReader {
            CustomReader { buf: s, idx: 0 }
        }
    }

    impl Read for CustomReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.idx < self.buf.len() {
                let len = std::cmp::min(buf.len(), self.buf.len() - self.idx);
                buf[..len].copy_from_slice(&self.buf.as_bytes()[self.idx..self.idx + len]);
                self.idx += len;
                Ok(len)
            } else {
                Ok(0)
            }
        }
    }

    impl BufRead for CustomReader {
        fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
            Ok(self.buf.as_bytes())
        }

        fn consume(&mut self, amt: usize) {
            self.idx += amt;
        }
    }
}
