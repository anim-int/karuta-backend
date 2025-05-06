FROM rust AS builder
WORKDIR /usr/src/karuta-backend
COPY src src
COPY Cargo.toml Cargo.toml
RUN cargo install --path .

FROM debian
WORKDIR /usr/local/karuta-backend
COPY --from=builder /usr/local/cargo/bin/karuta_backend /usr/local/bin/karuta_backend
COPY karuta.toml karuta.toml
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["karuta_backend"]
