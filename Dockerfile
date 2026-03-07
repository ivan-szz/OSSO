#stage 1
FROM rust:1.91 as builder
WORKDIR /app
COPY . .
RUN cargo build --release


#stage 2
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/osso .
EXPOSE 8080
CMD ["./osso"]