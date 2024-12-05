FROM rust:1-alpine3.19 as builder

COPY ./ ./
RUN cargo check
RUN cargo test
RUN cargo build --release



FROM alpine:3.19 as runner

COPY --from=builder /target/release/abc /usr/local/bin/abc

ENTRYPOINT ["abc"]