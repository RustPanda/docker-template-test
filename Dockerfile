FROM rust:1.64.0-alpine3.16 AS builder

RUN apk --no-cache add build-base && \
    rustup default nightly && \
    cargo search tokio
    
COPY . .

RUN RUSTFLAGS='-C target-feature=+crt-static' \
    cargo build --release \
    && strip target/release/docker-template-test

FROM scratch
COPY --from=builder target/release/docker-template-test /docker-template-test
CMD ["/docker-template-test"]