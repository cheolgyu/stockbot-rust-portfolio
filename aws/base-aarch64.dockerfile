FROM rust:1.90 AS base

RUN apt update && \
    apt install --no-install-recommends -y \
    g++-aarch64-linux-gnu \
    libc6-dev-arm64-cross \
    glibc-source \
    wget    \
    && apt-get clean  \
    && rustup target add aarch64-unknown-linux-gnu \
    && rustup toolchain install stable-aarch64-unknown-linux-gnu  --force-non-host\
    && rm -rf  /var/lib/apt/lists/* /tmp/* /var/tmp/* \
    && cargo install sccache --locked

ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
    CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
    CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
RUN rustc --version
########################################################################################################
# aws 액세스키 입력
#   ``` aws configure ```
# ecr 로그인
#   ```aws ecr get-login-password --region ap-northeast-2 | docker login --username AWS --password-stdin 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/cheolgyucom/stockbot```
#
#  docker build -f ./aws/base-aarch64.dockerfile -t 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest . 
#  docker push 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest
#     
# 