#!/bin/bash

printf "\e[01;36m==> cloning repo to ~/.hyeong/hyeong\e[0m \n"
git clone https://github.com/buttercrab/hyeo-ung-lang ~/.hyeong/hyeong || (cd ~/.hyeong/hyeong && git pull)

printf "\n\e[01;36m==> making directory for building hyeong code\e[0m \n"
if [ ! -d "$HOME/.hyeong/hyeong-build/" ]; then
  cd ~/.hyeong && cargo new hyeong-build --vcs none
fi
cp ~/.hyeong/hyeong/src/number.rs ~/.hyeong/hyeong-build/src/
cp ~/.hyeong/hyeong/src/big_number.rs ~/.hyeong/hyeong-build/src/
printf "pub mod big_number;\npub mod number;" > ~/.hyeong/hyeong-build/src/lib.rs

printf "\n\e[01;36m==> test build for building hyeong code\e[0m \n"
cargo build --manifest-path="$HOME"/.hyeong/hyeong-build/Cargo.toml --release

printf "\n\e[01;36m==> done!\e[0m \n"

if [[ ":$PATH:" != *":$HOME/.hyeong/hyeong/target/release:"* ]]; then
  echo "run next to add to your PATH:"
  echo "    echo \"export PATH=\"\\\$PATH\":~/.hyeong/hyeong/target/release\" >> ~/.bashrc"
fi
