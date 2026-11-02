FROM 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest AS builder

RUN export \
    && sccache --version \
    && rustc --version



WORKDIR /stockbot
COPY batch/src batch/src
COPY batch/Cargo.toml batch/Cargo.toml
COPY batch_model batch_model
COPY batch_client batch_client
COPY utils utils

COPY .env.prod . 
COPY .env.batch_plan-init .env.batch_plan

WORKDIR /stockbot/batch
RUN cargo build --release --verbose --target aarch64-unknown-linux-gnu --features vendored-openssl

#######

########################################################################################################
# aws 액세스키 입력
#   ``` aws configure ```
# ecr 로그인
#   ```aws ecr get-login-password --region ap-northeast-2 | docker login --username AWS --password-stdin 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/cheolgyucom/stockbot```
#
#  docker build --no-cache -f ./aws/base-aarch64-batch.dockerfile -t 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest-batch . 
#  docker push 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest-batch
#     
# 