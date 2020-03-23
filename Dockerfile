FROM rust

RUN curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/v0.1.2/install_hyeong.sh" | /bin/bash
ENV PATH="${PATH}:~/.hyeong/hyeong/target/release"