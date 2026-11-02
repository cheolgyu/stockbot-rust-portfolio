FROM 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest as base

#ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
#ENV SCCACHE_CACHE_SIZE=90G
ENV SCCACHE_BUCKET=example-stockbot-cache
ENV SCCACHE_S3_KEY_PREFIX=sccache/
ENV AWS_ACCESS_KEY_ID=
ENV AWS_SECRET_ACCESS_KEY=
ENV SCCACHE_REGION=ap-northeast-2
#ENV SCCACHE_DIR=/tmp/sccache
RUN export \
    && sccache --version
WORKDIR /stockbot
COPY site-api site-api
COPY batch_model batch_model
COPY site-common site-common
COPY .env.prod .

WORKDIR /stockbot/site-api

RUN cargo build --release --verbose --target aarch64-unknown-linux-gnu 


FROM arm64v8/ubuntu as run
WORKDIR /stockbot
COPY --from=base /stockbot/site-api/target/aarch64-unknown-linux-gnu/release/site-api .
COPY --from=base /stockbot/.env.prod .
EXPOSE 80
CMD [ "./site-api" ]
#########################
# docker build  --progress=plain  -f site-api/test-sccache.dockerfile -t test-sccache-site-api:latest .
########################
