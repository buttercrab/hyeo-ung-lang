@echo off

cls
echo [01;36m==^> cloning repo to %%USERPROFILE%%\.hyeong\hyeong[0m
IF EXIST %USERPROFILE%\.hyeong\hyeong (
 cd %USERPROFILE%\.hyeong\hyeong && git pull
) else (
 git clone https://github.com/buttercrab/hyeo-ung-lang %USERPROFILE%\.hyeong\hyeong
)

echo [01;36m==^> building hyeong[0m
cargo install --path %USERPROFILE%/.hyeong/hyeong --root %USERPROFILE%/.hyeong

%USERPROFILE%\.hyeong\bin\hyeong.exe install
