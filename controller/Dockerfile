FROM rust:1

WORKDIR /betterjenkins

COPY ./src ./src
COPY ./web ./web
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/betterjenkins"]