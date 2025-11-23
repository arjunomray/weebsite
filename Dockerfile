# Build stage
FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Final run stage
FROM debian:bookworm-slim AS runner

WORKDIR /app
COPY --from=builder /app/target/release/weeb-site /app/weeb-site
COPY --from=builder /app/src/static /app/src/static
COPY --from=builder /app/blogs /app/blogs
CMD ["/app/weeb-site"]
