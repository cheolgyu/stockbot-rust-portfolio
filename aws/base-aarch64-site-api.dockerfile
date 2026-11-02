FROM 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest AS base

RUN export \
    && sccache --version


WORKDIR /stockbot
COPY site-api site-api 
COPY batch_model batch_model
COPY site-common site-common
COPY utils utils

COPY .env.prod . 

WORKDIR /stockbot/site-api 

RUN cargo build --release --verbose --target aarch64-unknown-linux-gnu
#######
########################################################################################################
# aws 액세스키 입력
#   ``` aws configure ```
# ecr 로그인
#   ```aws ecr get-login-password --region ap-northeast-2 | docker login --username AWS --password-stdin 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/cheolgyucom/stockbot```
#
#  docker build -f ./aws/base-aarch64-site-api.dockerfile -t 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest-site-api . 
#  docker push 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest-site-api
#     
# 