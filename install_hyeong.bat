@ECHO OFF

print "\e[01;36m==> cloning repo to ~/.hyeong/hyeong\e[0m \n"
git clone https://github.com/buttercrab/hyeo-ung-lang ~/.hyeong/hyeong || (cd %USERPROFILE%/.hyeong/hyeong && git pull)

print "\n\e[01;36m==> building hyeong\e[0m \n"
cargo build --manifest-path=%USERPROFILE%/.hyeong/hyeong/Cargo.toml --release

print "\n\e[01;36m==> making directory for building hyeong code\e[0m \n"
IF NOT EXIST %USERPROFILE%/.hyeong/hyeong-build/ (
    cd %USERPROFILE%/.hyeong && cargo new hyeong-build --vcs none
) 

COPY "%USERPROFILE%/.hyeong/hyeong/src/number.rs" "%USERPROFILE%/.hyeong/hyeong-build/src/"
COPY "%USERPROFILE%/.hyeong/hyeong/src/big_number.rs" "%USERPROFILE%/.hyeong/hyeong-build/src/"
PRINT "pub mod big_number;\npub mod number;" > %USERPROFILE%/.hyeong/hyeong-build/src/lib.rs

PRINT "\n\e[01;36m==> test build for building hyeong code\e[0m \n"
cargo build --manifest-path="%USERPROFILE%"/.hyeong/hyeong-build/Cargo.toml --release

PRINT "\n\e[01;36m==> done!\e[0m \n"