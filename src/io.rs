use anyhow::Result;
use std::io::BufRead;

// /// Custom writer structure for redirecting output.
// ///
// /// # Examples
// ///
// /// ```
// /// use std::io::Write;
// /// use hyeong::util::io::CustomWriter;
// ///
// /// let mut  a = CustomWriter::new(|_| Result::Ok(()));
// ///
// /// a.write_all("Hello, World!".as_bytes()).unwrap();
// /// assert_eq!("Hello, World!", a.to_string().unwrap());
// /// ```
// pub struct CustomWriter<T>
// where
//     T: Fn(String) -> std::io::Result<()>,
// {
//     buf: Vec<u8>,
//     print_fn: T,
// }
//
// impl<T> Write for CustomWriter<T>
// where
//     T: Fn(String) -> std::io::Result<()>,
// {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         self.buf.append(&mut buf.to_vec());
//         Ok(buf.len())
//     }
//
//     fn flush(&mut self) -> std::io::Result<()> {
//         let res = (self.print_fn)(self.to_string()?);
//         self.buf = Vec::new();
//         res
//     }
// }
//
// impl<T> CustomWriter<T>
// where
//     T: Fn(String) -> std::io::Result<()>,
// {
//     /// Make new `CustomWriter`
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use std::io::Write;
//     /// use hyeong::util::io::CustomWriter;
//     ///
//     /// let mut  a = CustomWriter::new(|_| Result::Ok(()));
//     ///
//     /// a.write_all("Hello, World!".as_bytes()).unwrap();
//     /// assert_eq!("Hello, World!", a.to_string().unwrap());
//     /// ```
//     pub fn new(func: T) -> CustomWriter<T> {
//         CustomWriter {
//             buf: Vec::new(),
//             print_fn: func,
//         }
//     }
//
//     /// Return string that is written
//     pub fn to_string(&self) -> std::io::Result<String> {
//         String::from_utf8(self.buf.clone()).map_err(|e| std::io::Error::new(ErrorKind::Other, e))
//     }
// }

/// ReadLine trait that is used in reading lines
/// This is made for uniting `std::io::Stdin` and `CustomReader`
pub trait ReadLine {
    fn read_line_(&mut self) -> Result<String>;
}

impl ReadLine for std::io::StdinLock<'_> {
    /// `read_line` wrapper
    #[cfg(not(tarpaulin_include))]
    fn read_line_(&mut self) -> Result<String> {
        let mut res = String::new();
        self.read_line(&mut res)?;
        Ok(res)
    }
}

// /// Custom reader structure for other input source
// ///
// /// # Examples
// ///
// /// ```
// /// use hyeong::util::io::{CustomReader, ReadLine};
// ///
// /// let mut a = CustomReader::new(String::from("Hello, World!"));
// ///
// /// assert_eq!("Hello, World!", a.read_line_().unwrap());
// /// ```
// pub struct CustomReader {
//     buf: Vec<String>,
//     idx: usize,
// }

// impl ReadLine for CustomReader {
//     /// Reads until next line character(`\n`)
//     fn read_line_(&mut self) -> Result<String> {
//         if self.buf.len() == self.idx {
//             Ok(String::from(""))
//         } else {
//             let res = self.buf[self.idx].clone();
//             self.idx += 1;
//             Ok(res)
//         }
//     }
// }
//
// /// Read line from `ReadLine`
// // pub fn read_line_from(input: &mut impl ReadLine) -> Result<String> {
// //     input.read_line_()
// // }
//
// impl CustomReader {
//     /// Make new `CustomReader`
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use hyeong::util::io::{CustomReader, ReadLine};
//     ///
//     /// let mut a = CustomReader::new(String::from("Hello, World!"));
//     ///
//     /// assert_eq!("Hello, World!", a.read_line_().unwrap());
//     /// ```
//     pub fn new(s: String) -> CustomReader {
//         CustomReader {
//             buf: s.split('\n').map(String::from).collect(),
//             idx: 0,
//         }
//     }
// }
