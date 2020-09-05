FROM rust
MAINTAINER "Jaeyong Sung"
LABEL version="0.2.2"

RUN curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/v0.2.2/install_hyeong.sh" | /bin/bash
ENV PATH="${PATH}:~/.hyeong/hyeong/target/release"