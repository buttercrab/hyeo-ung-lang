#[cfg(test)]
mod execute_test {
    use std::io::{Error, Result, Write};

    use hyeong::{execute, parse};
    use hyeong::code::UnOptState;

    struct CustomWriter {
        buffer: Vec<u8>,
    }

    impl Write for CustomWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.append(&mut buf.to_vec());
            Result::Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Result::Ok(())
        }
    }

    impl CustomWriter {
        fn new() -> CustomWriter {
            CustomWriter {
                buffer: Vec::new(),
            }
        }
    }

    fn helper_function(code: &str, stdout: &str, stderr: &str) -> bool {
        let parsed = parse::parse(code.to_string());
        let mut out = CustomWriter::new();
        let mut err = CustomWriter::new();
        let mut state = UnOptState::new();

        for c in parsed {
            state = execute::execute(&mut out, &mut err, state, &c);
        }

        stdout.as_bytes().to_vec() == out.buffer && stderr.as_bytes().to_vec() == err.buffer
    }

    #[test]
    fn execute_test01() {
        assert!(helper_function("혀어어어어어어엉......핫.", "0", ""));
    }
}