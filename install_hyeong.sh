echo "==> cloning repo to ~/.hyeong"
git clone https://github.com/buttercrab/hyeo-ung-lang ~/.hyeong
echo "==> building hyeong"
cargo build --manifest-path=$HOME/.hyeong/Cargo.toml
echo "==> done!"
echo "run next to add to your PATH:"
echo "    echo \"export PATH=\$PATH:~/.hyeong/target/debug\" >> ~/.bashrc"
