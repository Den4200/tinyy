##### Build Image #####
FROM rust:1.50 as builder

# Don't download Rust docs
RUN rustup set profile minimal

# Install DB packages
RUN apt-get update && apt-get install -y \
    --no-install-recommends \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Create dummy project to build and install dependencies
RUN USER=root cargo new --bin /app
WORKDIR /app

# Copy over manifests and build files
COPY ./Cargo.* ./
COPY ./rust-toolchain ./rust-toolchain

# Build dependencies and removes the dummy project
# except for the target folder
RUN cargo build --release
RUN find . -not -path "./target*" -delete

# Copy the entire project
COPY . .

# Build full project
RUN cargo build --release

##### Runtime Image #####
FROM debian:buster-slim

LABEL org.opencontainers.image.source=https://github.com/Den4200/tinyy

# Install packages
RUN apt-get update && apt-get install -y \
    --no-install-recommends \
    openssl \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copy files from context and the built binary
COPY Rocket.toml .
COPY --from=builder /app/target/release/tinyy_backend .

CMD ["./tinyy_backend"]
