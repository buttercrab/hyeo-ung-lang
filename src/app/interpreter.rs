use crate::core::state::UnOptState;
use crate::core::{execute, parse};
use crate::util::error::Error;
use crate::util::io;
use crate::util::option::HyeongOption;
use std::io::{stdin, Write};
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

/// Interpreter
///
/// It gets code line by line and executes.
/// Prints stdout and stderr separately.
#[cfg(not(tarpaulin_include))]
pub fn run(stdout: &mut StandardStream, hy_opt: &HyeongOption) -> Result<(), Error> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let color = hy_opt.color;
    let mut state = UnOptState::new();

    ctrlc::set_handler(move || {
        if r.load(Ordering::SeqCst) {
            r.store(false, Ordering::SeqCst);
            let mut stdout = StandardStream::stdout(color);
            write!(stdout, "\ntype \"흑.하앙...\" or \"exit\" to exit\n").unwrap();
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))
                .unwrap();
            write!(stdout, ">").unwrap();
            stdout.reset().unwrap();
            write!(stdout, " ").unwrap();
            stdout.flush().unwrap();
            r.store(true, Ordering::SeqCst);
        }
    })
    .expect("Error setting Ctrl-C handler");

    writeln!(stdout, "Hyeo-ung Programming Language")?;
    writeln!(stdout, "type help for help")?;

    loop {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)).set_bold(true))?;
        write!(stdout, ">")?;
        stdout.reset()?;
        write!(stdout, " ")?;
        stdout.flush()?;
        running.store(true, Ordering::SeqCst);
        let input = io::read_line_from(&mut std::io::stdin())?;
        running.store(false, Ordering::SeqCst);

        if input == *"" {
            process::exit(0);
        }

        let mut out = io::CustomWriter::new(|x| {
            if !x.is_empty() {
                let mut stdout = StandardStream::stdout(hy_opt.color);
                write!(stdout, "[")?;
                stdout.set_color(ColorSpec::new().set_bold(true))?;
                write!(stdout, "stdout")?;
                stdout.reset()?;
                writeln!(stdout, "] {}", x)?;
                stdout.flush()?;
            }

            Ok(())
        });

        let mut err = io::CustomWriter::new(|x| {
            if !x.is_empty() {
                let mut stdout = StandardStream::stdout(hy_opt.color);
                write!(stdout, "[")?;
                stdout.set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Red)))?;
                write!(stdout, "stderr")?;
                stdout.reset()?;
                writeln!(stdout, "] {}", x)?;
                stdout.flush()?;
            }

            Ok(())
        });

        match input.trim() {
            "" => {
                continue;
            }

            "clear" => {
                state = UnOptState::new();
            }

            "help" => {
                writeln!(stdout, "clear  Clears the state")?;
                writeln!(stdout, "exit   Exit this interpreter")?;
                writeln!(stdout, "       You can also exit by typing \"흑.하앙...\"")?;
                writeln!(stdout, "help   Print this")?;
                continue;
            }

            "exit" => {
                process::exit(0);
            }

            _ => {
                let code = parse::parse(input);
                for c in code.iter() {
                    state = execute::execute(&mut stdin(), &mut out, &mut err, state, c)?;
                }
            }
        }

        out.flush().unwrap();
        err.flush().unwrap();
    }
}
