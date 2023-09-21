FROM rust:1.72-alpine3.18 as builder

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN apk add pkgconfig openssl-dev musl-dev openssl

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Build the app
RUN cargo leptos build --release -vv

FROM rust:1.70-alpine as runner
# Copy the server binary to the /app directory
COPY --from=builder /app/target/server/release/leptos_leverage_calculator /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY --from=builder /app/target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="target/site"
EXPOSE 3000
# Run the server
CMD ["/app/leptos_leverage_calculator"]
