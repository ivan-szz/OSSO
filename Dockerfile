#stage 1
FROM rust:1.91 as builder
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos
COPY . .
RUN cargo leptos build --release


#stage 2
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/osso .
EXPOSE 8080
CMD ["./osso"]