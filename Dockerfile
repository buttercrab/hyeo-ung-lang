FROM rust
MAINTAINER "Jaeyong Sung"
LABEL version="0.1.3"

RUN curl "https://raw.githubusercontent.com/buttercrab/hyeo-ung-lang/v0.1.3/install_hyeong.sh" | /bin/bash
ENV PATH="${PATH}:~/.hyeong/hyeong/target/release"