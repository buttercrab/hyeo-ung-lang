FROM rust

RUN curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/master/install_hyeong.sh" | /bin/bash
ENV PATH="${PATH}:~/.hyeong/hyeong/target/release"