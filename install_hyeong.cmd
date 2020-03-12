@echo off

cls
echo [01;36m==^> cloning repo to ~/.hyeong/hyeong[0m
IF EXIST %USERPROFILE%\.hyeong\hyeong (
 cd %USERPROFILE%\.hyeong\hyeong && git pull
) else (
 git clone https://github.com/buttercrab/hyeo-ung-lang %USERPROFILE%\.hyeong\hyeong
)

echo [01;36m==^> building hyeong[0m
cargo build --manifest-path=%USERPROFILE%\.hyeong\hyeong\Cargo.toml --release

echo [01;36m==^> making directory for building hyeong code[0m
IF NOT EXIST %USERPROFILE%\.hyeong\hyeong-build (
 cd %USERPROFILE%\.hyeong && cargo new hyeong-build --vcs none
)

COPY /y %USERPROFILE%\.hyeong\hyeong\src\number.rs %USERPROFILE%\.hyeong\hyeong-build\src\number.rs
copy /y %USERPROFILE%\.hyeong\hyeong\src\big_number.rs %USERPROFILE%\.hyeong\hyeong-build\src\big_number.rs
echo pub mod big_number;pub mod number; > %USERPROFILE%\.hyeong\hyeong-build\src\lib.rs

echo [01;36m==^> test build for building hyeong code[0m
cargo build --manifest-path=%USERPROFILE%\.hyeong\hyeong-build\Cargo.toml --release

echo [01;36m==^> done![0m