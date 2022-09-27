FROM rust:1.64 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates
COPY --from=builder /usr/local/cargo/bin/yaoya /usr/local/bin/yaoya
CMD ["yaoya"]
