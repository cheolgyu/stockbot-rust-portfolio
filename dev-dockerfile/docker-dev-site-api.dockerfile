FROM ubuntu:latest as pre
RUN apt-get update -y && \
    apt-get -y install curl gcc make build-essential && \
    apt-get update -y

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default  1.70

RUN mkdir -p /build

RUN cargo install cargo-watch

RUN rustc --version &&  cargo --version



FROM pre as dev
ENV CARGO_TERM_COLOR=always
ENV CARGO_TARGET_DIR=/build
WORKDIR /stockbot/site-api
EXPOSE 8081
ENTRYPOINT cargo watch -x 'run -- --color=always'

