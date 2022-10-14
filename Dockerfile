FROM rust:alpine AS builder

COPY . .

RUN apk add build-base
RUN cargo build --release

FROM scratch
COPY --from=builder target/release/docker-template-test /docker-template-test
CMD ["/docker-template-test"]