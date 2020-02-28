echo -e "\033[01;36m==> cloning repo to ~/.hyeong\033[0m"
git clone https://github.com/buttercrab/hyeo-ung-lang ~/.hyeong
echo -e "\n\033[01;36m==> building hyeong\033[0m"
cargo build --manifest-path=$HOME/.hyeong/Cargo.toml
echo -e "\n\033[01;36m==> done!\033[0m"
echo "run next to add to your PATH:"
echo "    echo \"export PATH=\$PATH:~/.hyeong/target/debug\" >> ~/.bashrc"
