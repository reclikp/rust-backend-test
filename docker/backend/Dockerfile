FROM rust:1.85.1

WORKDIR /app

RUN cargo install cargo-watch

EXPOSE 8000

#COPY ../backend/Cargo.toml ./

#RUN find ../backend/ -name 'Cargo.toml' | cpio -pdm ./

#RUN cargo fetch

COPY ../backend .

CMD ["cargo", "watch", "--poll", "-x", "run"]
