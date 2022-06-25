# Create the build container to compile the hello world program
FROM rust:1.61.0-buster as builder
RUN apt-get update && apt-get install -y cmake musl-tools && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /gotify-auth-proxy
COPY . .
RUN cargo build --target=x86_64-unknown-linux-musl --release

# Create the execution container by copying the compiled binary
FROM scratch
WORKDIR /gotify-auth-proxy
COPY --from=builder /gotify-auth-proxy/target/x86_64-unknown-linux-musl/release/gotify-auth-proxy /gotify-auth-proxy/gotify-auth-proxy
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
CMD ["/gotify-auth-proxy/gotify-auth-proxy"]