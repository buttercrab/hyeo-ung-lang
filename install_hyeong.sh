#!/bin/bash

printf "\e[01;36m==> cloning repos to ~/.hyeong\e[0m \n"
git clone https://github.com/buttercrab/hyeo-ung-lang ~/.hyeong/hyeong || (cd ~/.hyeong/hyeong && git pull)

printf "\n\e[01;36m==> building hyeong\e[0m \n"
cargo build --manifest-path="$HOME"/.hyeong/hyeong/Cargo.toml --release

printf "\n\e[01;36m==> done!\e[0m \n"

if [[ ":$PATH:" == *":$HOME/.hyeong/hyeong/target/release:"* ]]; then
  echo "run next to add to your PATH:"
  echo "    echo \"export PATH=\"\\\$PATH\":~/.hyeong/hyeong/target/release\" >> ~/.bashrc"
fi
