FROM rust:1

WORKDIR /betterjenkins

COPY ./ ./

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/betterjenkins"]