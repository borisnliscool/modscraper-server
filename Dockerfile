FROM rust:1.79-slim-buster

COPY ./ ./
RUN cargo build --release

EXPOSE 3000
CMD ["./target/release/modscraper-server"]