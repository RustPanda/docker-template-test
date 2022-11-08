FROM rust:alpine AS builder

RUN apk --no-cache add build-base && \
    rustup default nightly && \
    cargo search tokio
    
COPY . .

RUN RUSTFLAGS='-C target-feature=+crt-static' \
    cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/docker-template-test


FROM scratch
COPY --from=builder target/x86_64-unknown-linux-musl/release/docker-template-test /docker-template-test
CMD ["/docker-template-test"]