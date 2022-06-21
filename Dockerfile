FROM rust:1.61 as builder
WORKDIR /usr/src/figmaid
COPY . .
RUN cargo clean && \
    cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/figmaid /usr/local/bin/figmaid

ENV HOST=0.0.0.0

EXPOSE 18412

CMD ["sh", "-c", "HOST=${HOST} figmaid"]