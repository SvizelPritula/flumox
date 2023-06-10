FROM docker.io/library/rust:1.70-alpine3.18 AS cargo

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/flumox

COPY time-expr/ ./time-expr/
COPY flumox/ ./flumox/
COPY flumox-server/ ./flumox-server/
COPY Cargo.toml Cargo.lock ./

RUN [ "cargo", "build", "--release", "--bin", "flumox-server" ]

FROM docker.io/library/alpine:3.18 AS runtime

RUN apk add --no-cache tini

COPY --from=cargo /usr/src/flumox/target/release/flumox-server /usr/local/bin/

EXPOSE 3000

ENTRYPOINT [ "tini", "flumox-server" ]
