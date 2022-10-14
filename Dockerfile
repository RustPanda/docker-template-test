FROM rust:alpine AS builder

COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder target/release/docker-template-test /docker-template-test
CMD ["/docker-template-test"]