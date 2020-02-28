printf "\e[01;36m==> cloning repo to ~/.hyeong\e[0m\n\n\n"
git clone https://github.com/buttercrab/hyeo-ung-lang ~/.hyeong
printf "\n\e[01;36m==> building hyeong\e[0m\n\n\n"
cargo build --manifest-path="$HOME"/.hyeong/Cargo.toml
printf "\n\e[01;36m==> done!\e[0m\n"
echo "run next to add to your PATH:"
echo "    echo \"export PATH=\$PATH:~/.hyeong/target/debug\" >> ~/.bashrc"
