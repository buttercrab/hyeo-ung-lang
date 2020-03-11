@echo off

cls
echo ==^> cloning repo to ~/.hyeong/hyeong
IF EXIST %USERPROFILE%\.hyeong\hyeong (
 cd %USERPROFILE%\.hyeong\hyeong && git pull
) else (
 git clone https://github.com/buttercrab/hyeo-ung-lang %USERPROFILE%\.hyeong\hyeong
)

echo ==^> building hyeong
cargo build --manifest-path=%USERPROFILE%\.hyeong\hyeong\Cargo.toml --release

echo ==^> making directory for building hyeong code
IF NOT EXIST %USERPROFILE%\.hyeong\hyeong-build (
 cd %USERPROFILE%\.hyeong && cargo new hyeong-build --vcs none
)

COPY %USERPROFILE%\.hyeong\hyeong\src\number.rs %USERPROFILE%\.hyeong\hyeong-build\src\number.rs
copy %USERPROFILE%\.hyeong\hyeong\src\big_number.rs %USERPROFILE%\.hyeong\hyeong-build\src\big_number.rs
echo pub mod big_number;pub mod number; > %USERPROFILE%\.hyeong\hyeong-build\src\lib.rs

echo ==^> test build for building hyeong code
cargo build --manifest-path=%USERPROFILE%\.hyeong\hyeong-build\Cargo.toml --release

echo ==^> done!