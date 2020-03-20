use crate::{io, option};
use clap::{App, ArgMatches};

pub fn install_app<'a, 'b>() -> App<'a, 'b> {
    App::new("install")
        .about("Install hyeong before build (need once)")
        .arg(option::build_source())
}

pub fn uninstall_app<'a, 'b>() -> App<'a, 'b> {
    App::new("uninstall")
        .about("Uninstall hyeong temporary build path")
        .arg(option::build_source())
}

pub fn install_run(matches: &ArgMatches) {
    io::print_log("installing hyeong");
    io::execute_command_stderr(
        "\
            mkdir %USERPROFILE%\\.hyeong\n\
            cd %USERPROFILE%\\.hyeong && cargo new hyeong-build --vcs none\n\
            curl \"https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/src/number.rs\" > %USERPROFILE%\\.hyeong\\hyeong-build\\src\\number.rs;\
            curl \"https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/src/big_number.rs\" > %USERPROFILE%\\.hyeong\\hyeong-build\\src\\big_number.rs;\
            echo pub mod big_number;pub mod number; > %USERPROFILE%\\.hyeong\\hyeong-build\\src\\lib.rs",
        "\
            mkdir -p ~/.hyeong;\
            cd ~/.hyeong && cargo new hyeong-build --vcs none --color always;\
            curl \"https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/src/number.rs\" > ~/.hyeong/hyeong-build/src/number.rs;\
            curl \"https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/src/big_number.rs\" > ~/.hyeong/hyeong-build/src/big_number.rs;\
            printf \"pub mod big_number;\npub mod number;\" > ~/.hyeong/hyeong-build/src/lib.rs",
    );
    io::print_log("test build");
    io::execute_command_stderr(
        "\
            cargo build --manifest-path=%USERPROFILE%\\.hyeong\\hyeong-build\\Cargo.toml --release",
        "\
            cargo build --manifest-path=\"$HOME\"/.hyeong/hyeong-build/Cargo.toml --release --color always",
    );
    io::print_log("done!");
    io::print_note("to uninstall, run `hyeong uninstall`");
}

pub fn uninstall_run(matches: &ArgMatches) {
    io::print_log("uninstalling hyeong");
    io::execute_command_stdout("rmdir /S %USERPROFILE%\\.hyeong", "rm -rf ~/.hyeong");
    io::print_log("done!");
}
