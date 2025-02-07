#!/bin/sh -e

AWS_DEFAULT_REGION="elasticmq" \
AWS_ACCESS_KEY_ID="x" \
AWS_SECRET_ACCESS_KEY="x" \
PUBSUB_EMULATOR_HOST=localhost:8085 \
PUBSUB_PROJECT_ID=local-project \
cargo test --all-features -- "$@"
