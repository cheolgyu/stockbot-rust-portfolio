FROM 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest as base
RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk 

RUN apt update -y && apt install -y \
    nodejs \
    npm \
    && npm install -g npx tailwindcss  && nodejs -v 

# ARG BACKEND_ADDR
ENV BACKEND_ADDR=http://stockbot-api.example.invalid:8080

#########################################################
#           sccache 
#########################################################
# ARG SCCACHE_BUCKET
# ARG SCCACHE_S3_KEY_PREFIX
# ARG AWS_ACCESS_KEY_ID
# ARG AWS_SECRET_ACCESS_KEY
# ARG SCCACHE_REGION

ENV SCCACHE_BUCKET=example-stockbot-cache
ENV SCCACHE_S3_KEY_PREFIX=sccache/
ENV AWS_ACCESS_KEY_ID=
ENV AWS_SECRET_ACCESS_KEY=
ENV SCCACHE_REGION=ap-northeast-2

RUN export \
    && sccache --version
#########################################################

WORKDIR /stockbot
COPY site site 
COPY site-common site-common


WORKDIR /stockbot/site 
COPY ../.env.prod . 

RUN trunk build --release 



#########################
# docker build  --progress=plain  -f site/test.dockerfile -t test-site:latest .
########################
