FROM rust:1-alpine3.19 as builder

COPY ./ ./


# RUN cargo check
# RUN cargo test
RUN apk add musl-dev
RUN cargo build --release



FROM alpine:3.19 as runner
RUN apk add libgcc
COPY --from=builder /target/release/abc /usr/local/bin/abc
EXPOSE 8080
ENTRYPOINT ["abc"]