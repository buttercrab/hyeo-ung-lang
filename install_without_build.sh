#!/bin/bash

mkdir -p ~/.hyeong

printf "\n\e[01;36m==> making directory for building hyeong code\e[0m \n"
if [ ! -d "$HOME/.hyeong/hyeong-build/" ]; then
  cd ~/.hyeong && cargo new hyeong-build --vcs none
fi
curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/src/number.rs" > ~/.hyeong/hyeong-build/src/number.rs
curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/src/big_number.rs" > ~/.hyeong/hyeong-build/src/big_number.rs
printf "pub mod big_number;\npub mod number;" > ~/.hyeong/hyeong-build/src/lib.rs

printf "\n\e[01;36m==> test build for building hyeong code\e[0m \n"
cargo build --manifest-path="$HOME"/.hyeong/hyeong-build/Cargo.toml --release

printf "\n\e[01;36m==> done!\e[0m \n"
