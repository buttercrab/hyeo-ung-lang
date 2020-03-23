#!/bin/bash

printf "\e[01;36m==> cloning repo to ~/.hyeong/hyeong\e[0m \n"
git clone https://github.com/buttercrab/hyeo-ung-lang ~/.hyeong/hyeong || (cd ~/.hyeong/hyeong && git pull)

printf "\n\e[01;36m==> building hyeong\e[0m \n"
cargo install --path "$HOME"/.hyeong/hyeong --root "$HOME"/.hyeong

~/.hyeong/bin/hyeong install

if [[ ":$PATH:" != *":$HOME/.hyeong/bin:"* ]]; then
  echo "run next to add to your PATH:"
  echo "    echo \"export PATH=\"\\\$PATH\":~/.hyeong/bin\" >> ~/.bashrc"
fi
