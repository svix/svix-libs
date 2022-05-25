#!/bin/sh -e

# Run tests with various configurations:

TEST_COMMAND="cargo test --all --all-features --all-targets"

# Common variables:
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/postgres"
export SVIX_JWT_SECRET="test value"

echo "*********** RUN 1 ***********"
SVIX_QUEUE_TYPE="redis" \
SVIX_CACHE_TYPE="redis" \
SVIX_REDIS_DSN="redis://localhost:6379" \
${TEST_COMMAND}

echo "*********** RUN 2 ***********"
SVIX_QUEUE_TYPE="redis" \
SVIX_CACHE_TYPE="memory" \
SVIX_REDIS_DSN="redis://localhost:6379" \
${TEST_COMMAND}

echo "*********** RUN 3 ***********"
SVIX_QUEUE_TYPE="redis" \
SVIX_CACHE_TYPE="none" \
SVIX_REDIS_DSN="redis://localhost:6379" \
${TEST_COMMAND}

echo "*********** RUN 4 ***********"
SVIX_QUEUE_TYPE="rediscluster" \
SVIX_CACHE_TYPE="rediscluster"  \
SVIX_REDIS_DSN="redis://localhost:6380" \
${TEST_COMMAND}

echo "*********** RUN 5 ***********"
SVIX_QUEUE_TYPE="memory" \
SVIX_CACHE_TYPE="none"  \
SVIX_REDIS_DSN="redis://localhost:6379" \
${TEST_COMMAND}


