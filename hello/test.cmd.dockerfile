FROM 000000000000.dkr.ecr.ap-northeast-2.amazonaws.com/stockbot-base:latest as base

WORKDIR /stockbot
COPY hello hello
COPY utils utils

WORKDIR /stockbot/hello

RUN cargo build  --verbose --target aarch64-unknown-linux-gnu

FROM arm64v8/ubuntu as run
WORKDIR /root
COPY --from=base /stockbot/hello/target/aarch64-unknown-linux-gnu/debug/hello . 
COPY --from=base /stockbot/hello/script/hello.sh .
RUN chmod +x ./hello.sh
EXPOSE 80
CMD [ "./hello" ]


