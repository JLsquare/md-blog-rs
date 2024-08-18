FROM rust:1.79 as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/app/target/release/md-blog-rs .

COPY --from=builder /usr/src/app/static ./static
COPY --from=builder /usr/src/app/markdown ./markdown
COPY --from=builder /usr/src/app/index.html .

EXPOSE 8080

ENV HOST=0.0.0.0
ENV PORT=8080
ENV RUST_LOG=INFO

CMD ["./md-blog-rs"]