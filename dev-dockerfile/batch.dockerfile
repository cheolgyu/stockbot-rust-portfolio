FROM rust:1.67 as builder
RUN ls -al
WORKDIR /stockbot
COPY . .
RUN cargo build --release --verbose --package batch --bin kr_timer --target-dir build
RUN cd build && ls -al

FROM  debian:bullseye-slim as prod
WORKDIR /stockbot
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /stockbot/build/release/kr_timer . 
COPY --from=builder /stockbot/.env.prod . 
# need env file
CMD [ "./kr_timer" ]