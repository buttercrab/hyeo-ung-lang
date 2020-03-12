FROM rust

RUN bash <(curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/install_hyeong.sh")
ENV PATH="${PATH}:~/.hyeong/hyeong/target/release"