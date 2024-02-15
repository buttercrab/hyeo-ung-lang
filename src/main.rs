use std::{path::PathBuf, sync::atomic::Ordering};

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use hyeong::{
    commands::{build, check, debug, interpret, run},
    error_barrier, ERROR_COUNT, WARN_COUNT,
};
use log::{error, warn, Level};

/// Hyeo-ung Programming Language Toolchain
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct App {
    /// set verbose output
    #[arg(long, global = true)]
    verbose: bool,
    /// whether prints with color
    #[arg(long, default_value = "auto", global = true, hide_default_value = true)]
    color: Color,
    /// If no subcommand, it will run interpreter
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// hyeong code compiler
    ///
    /// It compiles hyeong code to rust code and then compiles it to binary.
    #[command(name = "build")]
    Build {
        /// optimize level
        #[arg(short = 'O', long = "optimize", default_value_t = 2, value_parser(clap::value_parser ! (u8).range(0..=2)))]
        level: u8,
        /// input file to compile
        #[arg(required = true, value_name = "FILE.hyeong")]
        file: PathBuf,
    },
    /// hyeong code checker
    ///
    /// It will check syntax and type of hyeong code.
    #[command(name = "check")]
    Check {
        /// input file to compile
        #[arg(required = true, value_name = "FILE.hyeong")]
        file: PathBuf,
        /// print each command in raw or in parsed format
        #[arg(long)]
        raw: bool,
    },
    /// hyeong code debugger
    ///
    /// It will debug hyeong code.
    #[command(name = "debug")]
    Debug {
        /// input file to compile
        #[arg(required = true, value_name = "FILE.hyeong")]
        file: PathBuf,
    },
    /// hyeong code runner
    ///
    /// It will run hyeong code.
    #[command(name = "run")]
    Run {
        /// optimize level
        #[arg(short = 'O', long = "optimize", default_value_t = 2, value_parser = clap::value_parser ! (u8).range(0..=2))]
        level: u8,
        /// input file to compile
        #[arg(required = true, value_name = "FILE.hyeong")]
        file: PathBuf,
    },
}

/// whether prints color
#[derive(ValueEnum, Debug, Clone)]
enum Color {
    Never,
    Auto,
    Always,
}

/// Sets up logging
///
/// Uses fern to set up logging.
/// If `verbose` is true, then the log level is set to `Debug`.
fn setup_logger(verbose: bool) -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            match record.level() {
                Level::Error => {
                    ERROR_COUNT.fetch_add(1, Ordering::Release);
                    out.finish(format_args!("{}: {}", "error".red().bold(), message));
                }
                Level::Warn => {
                    WARN_COUNT.fetch_add(1, Ordering::Release);
                    out.finish(format_args!("{}: {}", "warn".yellow().bold(), message))
                }
                Level::Info => {
                    if record.target().is_empty() {
                        out.finish(format_args!("{message}"))
                    } else {
                        out.finish(format_args!("{:>11} {}", record.target().green().bold(), message))
                    }
                }
                Level::Debug => {
                    if record.target().is_empty() {
                        out.finish(format_args!("{}: {}", "debug".blue().bold(), message))
                    } else {
                        out.finish(format_args!("{:>11} {}", record.target().blue().bold(), message))
                    }
                }
                Level::Trace => out.finish(format_args!("{}: {}", "trace".purple().bold(), message)),
            };
        })
        .level(if verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

/// Main function of hyeong
///
/// It parses arguments and runs subcommands.
/// If there is no subcommand, it runs the interpreter.
/// If there is an error, it prints the error and exits.
/// If there is a warning, it prints the warning and exits.
/// If there is no error or warning, it exits with 0.
fn main() {
    // Parse arguments
    let app = App::parse();

    // setup logger color
    match app.color {
        Color::Never => colored::control::set_override(false),
        Color::Always => colored::control::set_override(true),
        _ => {}
    }

    // using expect since it is not possible to fail
    setup_logger(app.verbose).expect("failed to setup logger");

    // run subcommand
    match app.command {
        Some(Commands::Build { level, file }) => build(level, file),
        Some(Commands::Check { file, raw }) => check(file, raw),
        Some(Commands::Debug { file }) => debug(file),
        Some(Commands::Run { level, file }) => run(level, file),
        None => interpret(),
    }
    .unwrap_or_else(|e| {
        error!("{}", e);
    });

    // print warning
    let warning_count = WARN_COUNT.load(Ordering::SeqCst);
    if warning_count > 0 {
        warn!("{} warning(s) generated", warning_count);
    }

    // exit with 1 when error occurred
    error_barrier(format_args!("could not do the job due to previous error"));
}
