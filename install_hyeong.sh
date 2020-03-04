#!/bin/bash

printf "\e[01;36m==> cloning repo to ~/.hyeong\e[0m \n"
git clone https://github.com/buttercrab/hyeo-ung-lang ~/.hyeong || (cd ~/.hyeong && git pull)

printf "\n\e[01;36m==> building hyeong\e[0m \n"
cargo build --manifest-path="$HOME"/Cargo.toml --release

printf "\n\e[01;36m==> done!\e[0m \n"
echo "run next to add to your PATH:"
echo "    echo \"export PATH=\$PATH:~/.hyeong/target/release\" >> ~/.bashrc"
