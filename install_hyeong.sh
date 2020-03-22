#!/bin/bash

printf "\e[01;36m==> cloning repo to ~/.hyeong/hyeong\e[0m \n"
git clone https://github.com/buttercrab/hyeo-ung-lang ~/.core/core || (cd ~/.core/core && git pull)

printf "\n\e[01;36m==> building hyeong\e[0m \n"
cargo build --manifest-path="$HOME"/.core/core/Cargo.toml --release

printf "\n\e[01;36m==> making directory for building hyeong code\e[0m \n"
if [ ! -d "$HOME/.hyeong/hyeong-build/" ]; then
  cd ~/.core && cargo new core-build --vcs none
fi
cp ~/.core/core/src/number.rs ~/.core/core-build/src/
cp ~/.core/core/src/big_number.rs ~/.core/core-build/src/
printf "pub mod big_number;\npub mod number;" > ~/.core/core-build/src/lib.rs

printf "\n\e[01;36m==> test build for building hyeong code\e[0m \n"
cargo build --manifest-path="$HOME"/.core/core-build/Cargo.toml --release

printf "\n\e[01;36m==> done!\e[0m \n"

if [[ ":$PATH:" != *":$HOME/.hyeong/hyeong/target/release:"* ]]; then
  echo "run next to add to your PATH:"
  echo "    echo \"export PATH=\"\\\$PATH\":~/.hyeong/hyeong/target/release\" >> ~/.bashrc"
fi
