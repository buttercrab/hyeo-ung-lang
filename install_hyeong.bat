@ECHO OFF

print "\e[01;36m==> cloning repo to ~/.hyeong/hyeong\e[0m \n"
git clone https://github.com/buttercrab/hyeo-ung-lang ~/.hyeong/hyeong || (cd %USERPROFILE%/.hyeong/hyeong && git pull)

print "\n\e[01;36m==> building hyeong\e[0m \n"
cargo build --manifest-path="$HOME"/.hyeong/hyeong/Cargo.toml --release

print "\n\e[01;36m==> making directory for building hyeong code\e[0m \n"
if [ ! -d "$HOME/.hyeong/hyeong-build/" ]; then
  cd %USERPROFILE%/.hyeong && cargo new hyeong-build --vcs none
fi
copy "%USERPROFILE%/.hyeong/hyeong/src/number.rs" "%USERPROFILE%/.hyeong/hyeong-build/src/"
copy "%USERPROFILE%/.hyeong/hyeong/src/big_number.rs" "%USERPROFILE%/.hyeong/hyeong-build/src/"
print "pub mod big_number;\npub mod number;" > %USERPROFILE%/.hyeong/hyeong-build/src/lib.rs

print "\n\e[01;36m==> test build for building hyeong code\e[0m \n"
cargo build --manifest-path="$HOME"/.hyeong/hyeong-build/Cargo.toml --release

print "\n\e[01;36m==> done!\e[0m \n"