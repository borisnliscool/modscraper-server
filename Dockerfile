FROM rust:1.79-slim-buster

COPY ./ ./
RUN cargo build --release

EXPOSE 8080
CMD ["./target/release/modscraper-server"]