# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM ekidd/rust-musl-builder:nightly-2019-11-06-openssl11 as cargo-build

WORKDIR /usr/src/

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust \
    && sudo chown -R rust:rust /usr/src

COPY . .

RUN mkdir .cargo \
    && cargo vendor > .cargo/config

RUN rustup target add x86_64-unknown-linux-musl

RUN OPENSSL_STATIC=true \
    RUSTFLAGS=-Clinker=musl-gcc \
    cargo build --release --target=x86_64-unknown-linux-musl -p merit-api

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

ARG GH_ACCESS_TOKEN

ENV GH_ACCESS_TOKEN=${GH_ACCESS_TOKEN} \
    PORT=8080 \
    LOG_LEVEL="actix_web=info"

WORKDIR /home/merit/bin/

COPY --from=cargo-build /usr/src/target/x86_64-unknown-linux-musl/release/merit-api .

RUN addgroup -g 1000 merit \
    && adduser -D -s /bin/sh -u 1000 -G merit merit \
    && chown merit:merit merit-api

USER merit

EXPOSE ${PORT}

CMD ["./merit-api"]