FROM alpine:3.16 as builder

RUN apk add --update	curl \
						gcc \
						musl-dev

# Instalation of rust and cargo
RUN curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh -s -- -y
# RUN rustup update

WORKDIR /app
COPY . .
RUN $HOME/.cargo/bin/cargo build --release

RUN mv ./target/release/rust-notebook .

# TODO Debug
# ENV RUST_BACKTRACE=full


FROM alpine:3.16
WORKDIR /app
COPY --from=builder /app/docs/db.sql ./docs/
COPY --from=builder /app/rust-notebook .
ENTRYPOINT ["./rust-notebook", "/db/notebook.db"]
