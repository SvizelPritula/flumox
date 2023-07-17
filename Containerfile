FROM docker.io/library/rust:1.70-alpine3.18 AS cargo

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/flumox

COPY channel-map/ ./channel-map/
COPY time-expr/ ./time-expr/
COPY flumox/ ./flumox/
COPY flumox-server/ ./flumox-server/
COPY flumox-game-tracker/ ./flumox-game-tracker/
COPY flumox-seed-maker/ ./flumox-seed-maker/
COPY Cargo.toml Cargo.lock ./

RUN [ "cargo", "build", "--release", "--bin", "flumox-server" ]

FROM docker.io/library/node:20.3-alpine3.18 as vite

RUN npm config set update-notifier false

WORKDIR /usr/src/flumox

COPY flumox-client/*.json flumox-client/*.js flumox-client/*.ts ./

RUN npm ci --no-audit --no-fund

COPY flumox-client/src/ ./src/
COPY flumox-client/public/ ./public/
COPY flumox-client/index.html ./

RUN npm run build

FROM docker.io/library/alpine:3.18 AS runtime

RUN apk add --no-cache tini

COPY --from=cargo /usr/src/flumox/target/release/flumox-server /usr/local/bin/
COPY --from=vite /usr/src/flumox/dist/ /srv/flumox/www/

ENV PORT=8000
EXPOSE 8000

ENTRYPOINT [ "tini", "flumox-server", "--serve", "/srv/flumox/www/" ]
