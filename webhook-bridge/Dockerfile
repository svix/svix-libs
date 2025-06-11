# Base build
FROM rust:1.69-slim-bullseye AS build

RUN apt-get update && apt-get install -y \
    build-essential=12.* \
    checkinstall=1.* \
    zlib1g-dev=1:* \
    pkg-config=0.29.* \
    libssl-dev=* \
    protobuf-compiler=* \
    --no-install-recommends

RUN set -ex ; \
        mkdir -p /app ;\
        useradd appuser ;\
        chown -R appuser: /app ;\
        mkdir -p /home/appuser ;\
        chown -R appuser: /home/appuser

WORKDIR /app

# Hack to enable docker caching
COPY Cargo.toml .
COPY Cargo.lock .
COPY generic-queue/Cargo.toml generic-queue/
COPY svix-webhook-bridge-types/Cargo.toml svix-webhook-bridge-types/
COPY svix-webhook-bridge-plugin-queue-consumer/Cargo.toml svix-webhook-bridge-plugin-queue-consumer/
COPY svix-webhook-bridge-plugin-webhook-receiver/Cargo.toml svix-webhook-bridge-plugin-webhook-receiver/
COPY svix-webhook-bridge/Cargo.toml svix-webhook-bridge/
RUN set -ex ;\
        mkdir generic-queue/src ;\
        mkdir svix-webhook-bridge-plugin-queue-consumer/src ;\
        mkdir svix-webhook-bridge-plugin-webhook-receiver/src ;\
        mkdir svix-webhook-bridge-types/src ;\
        mkdir svix-webhook-bridge/src ;\
        echo '' > generic-queue/src/lib.rs ;\
        echo '' > svix-webhook-bridge-plugin-queue-consumer/src/lib.rs ;\
        echo '' > svix-webhook-bridge-plugin-webhook-receiver/src/lib.rs ;\
        echo '' > svix-webhook-bridge-types/src/lib.rs ;\
        echo 'fn main() { println!("Dummy!"); }' > svix-webhook-bridge/src/main.rs ;\
        cargo build --release ;\
        rm -rf \
          generic-queue/src \
          svix-webhook-bridge-plugin-queue-consumer/src \
          svix-webhook-bridge-plugin-webhook-receiver/src \
          svix-webhook-bridge-types/src \
          svix-webhook-bridge/src

COPY . .
# touching the lib.rs/main.rs ensures cargo rebuilds them instead of considering them already built.
RUN touch */src/lib.rs && touch */src/main.rs
RUN cargo build --release --frozen

# Production
FROM debian:11.2-slim AS prod

RUN set -ex ; \
        mkdir -p /app ;\
        useradd appuser ;\
        chown -R appuser: /app ;\
        mkdir -p /home/appuser ;\
        chown -R appuser: /home/appuser

RUN apt-get update ;\
    apt-get install --no-install-recommends -y ca-certificates=20210119; \
    update-ca-certificates; \
    rm -rf /var/lib/apt/lists/*

USER appuser

COPY --from=build /app/target/release/svix-webhook-bridge /usr/local/bin/svix-webhook-bridge

# Will fail if there's no `svix-webhook-bridge.yaml` in the CWD or `SVIX_WEBHOOK_BRIDGE_CFG` is not set to a valid
# path to a config.
CMD ["svix-webhook-bridge"]
