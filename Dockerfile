# Latest stable Rust
FROM rust:1.76.0

WORKDIR /app
RUN apt update && apt install lld clangd -y

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/zero"]
