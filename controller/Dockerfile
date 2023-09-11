FROM rust:1 as builder
WORKDIR /betterjenkins

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs

COPY ./src ./src
RUN touch src/main.rs
RUN cargo build --release

FROM rust:1 as runner

COPY --from=builder ./betterjenkins/target/release/betterjenkins ./target/release/betterjenkins
COPY ./web ./web
COPY ./k8s_manifests ./k8s_manifests

EXPOSE 8080

CMD ["./target/release/betterjenkins"]