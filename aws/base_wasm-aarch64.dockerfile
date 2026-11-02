FROM 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest AS builder

RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk 

RUN apt update -y && apt install -y \
    nodejs \
    npm \
    && npm install -g npx tailwindcss  && nodejs -v     

########################################################################################################
#  docker build -f ./aws/base_wasm-aarch64.dockerfile -t 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:wasm-latest .
#  docker push 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:wasm-latest