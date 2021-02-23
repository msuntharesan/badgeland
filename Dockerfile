# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:1.49-alpine3.12 as toolchain

RUN adduser -D -h /badgeland -g "" badgeland

RUN apk add make musl-dev

USER badgeland

ENV USER=badgeland
ENV PATH=/badgeland/.cargo/bin:$PATH

WORKDIR /badgeland

# Create skeleton dir for vendoring dependencies
RUN cargo init --lib --vcs none \
    && cargo new badgeland --lib --vcs none \
    && mkdir -p badgeland/src/bin \
    && touch badgeland/src/bin/cargo-badge.rs \
    && cargo new humanize --lib --vcs none \
    && cargo new badgeland-web --lib --vcs none \
    && rm -rf src

RUN ls -lah

# badgeland/Cargo.toml humanize/Cargo.toml badgeland-web/Cargo.toml ./

COPY Cargo.toml Cargo.lock ./ 
COPY badgeland/Cargo.toml ./badgeland/
COPY humanize/Cargo.toml ./humanize/
COPY badgeland-web/Cargo.toml ./badgeland-web/

# Vendor dependencies
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY . .

RUN cargo build --release -p badgeland-web

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

# FROM alpine:3.12
FROM scratch

ENV PORT=8080 \
    RUST_LOG="actix_web=info"

WORKDIR /home/badgeland/bin/

COPY --from=toolchain /etc/passwd /etc/passwd
COPY --from=toolchain /badgeland/target/release/badgeland-web .
COPY --from=toolchain /badgeland/badgeland-web/static/ .

EXPOSE ${PORT}

CMD ["./badgeland-web"]