FROM rust:1.85.1

WORKDIR /app

RUN cargo install cargo-watch

EXPOSE 8000

COPY ../backend/Cargo.toml ./

RUN cargo fetch

COPY ../backend .

CMD ["cargo", "watch", "--poll", "-x", "run"]
