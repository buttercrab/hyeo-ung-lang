@ECHO OFF

ECHO "\e[01;36m==> cloning repo to ~/.hyeong/hyeong\e[0m \n"
IF EXIST %USERPROFILE%\.hyeong\hyeong (
  cd %USERPROFILE%\.hyeong\hyeong && git pull
) else (
  git clone https://github.com/buttercrab/hyeo-ung-lang %USERPROFILE%\.hyeong\hyeong
)

ECHO "\n\e[01;36m==> building hyeong\e[0m \n"
cargo build --manifest-path=%USERPROFILE%\.hyeong\hyeong\Cargo.toml --release

ECHO "\n\e[01;36m==> making directory for building hyeong code\e[0m \n"
IF NOT EXIST %USERPROFILE%\.hyeong\hyeong-build (
    cd %USERPROFILE%\.hyeong && cargo new hyeong-build --vcs none
) 

COPY %USERPROFILE%\.hyeong\hyeong\src\number.rs %USERPROFILE%\.hyeong\hyeong-build\src\number.rs
copy %USERPROFILE%\.hyeong\hyeong\src\big_number.rs %USERPROFILE%\.hyeong\hyeong-build\src\big_number.rs
ECHO pub mod big_number;pub mod number; > %USERPROFILE%\.hyeong\hyeong-build\src\lib.rs

ECHO "\n\e[01;36m==> test build for building hyeong code\e[0m \n"
cargo build --manifest-path=%USERPROFILE%\.hyeong\hyeong-build\Cargo.toml --release

ECHO "\n\e[01;36m==> done!\e[0m \n"

