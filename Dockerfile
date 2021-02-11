# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:1.49-alpine3.12 as toolchain

RUN adduser -D -h /merit -g "" merit

RUN apk add make musl-dev

USER merit

ENV USER=merit
ENV PATH=/merit/.cargo/bin:$PATH

WORKDIR /merit

# Create skeleton dir for vendoring dependencies
RUN cargo init --lib --vcs none \
    && cargo new merit --lib --vcs none \
    && mkdir -p merit/src/bin \
    && touch merit/src/bin/cargo-badge.rs \
    && cargo new humanize --lib --vcs none \
    && cargo new merit-web --lib --vcs none

COPY Cargo.toml Cargo.lock ./
COPY merit/Cargo.toml ./merit
COPY humanize/Cargo.toml ./humanize
COPY merit-web/Cargo.toml ./merit-web

# Vendor dependencies
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY . .

RUN cargo build --release -p merit-web

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

# FROM alpine:3.12
FROM scratch

ENV PORT=8080 \
    RUST_LOG="actix_web=info"

WORKDIR /home/merit/bin/

COPY --from=toolchain /etc/passwd /etc/passwd
COPY --from=toolchain /merit/target/release/merit-web .
COPY --from=toolchain /merit/merit-web/static/ .

EXPOSE ${PORT}

CMD ["./merit-web"]