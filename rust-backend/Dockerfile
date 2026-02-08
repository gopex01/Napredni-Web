FROM rust:1.85-slim AS builder

WORKDIR /app

# Sistemske zavisnosti
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# 1. Kopiramo samo fajlove sa zavisnostima
COPY Cargo.toml Cargo.lock ./

# 2. Napravimo "lažni" main da bismo keširali zavisnosti
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/rust_backend*

# 3. Sada kopiramo pravi kod
COPY . .

# 4. Finalni build
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust-backend /app/rust-backend
EXPOSE 3000
CMD ["./rust-backend"]