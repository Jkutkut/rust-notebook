# FROM rust:alpine3.15
# FROM rust:alpine3.16
# FROM rust:latest
RUN rustup update
WORKDIR /app
COPY ./rust-notebook/ .
RUN cargo build --release

RUN mv ./target/release/rust-notebook .
ENV RUST_BACKTRACE=full
ENTRYPOINT ["./rust-notebook", "/db/notebook.db"]

# FROM alpine:3.14
# WORKDIR /app
# COPY --from=builder /usr/local/cargo/bin/rust-notebook .
# CMD ["./rust-notebook notebook.db"]
