#!/bin/bash

set -eo pipefail

OPENAPI_GIT_REV='272125558d6ac4718bdc87b1652e5d4122b69f19'

if [ -n "$1" ]; then
    curl "$1" | python -m json.tool > lib-openapi.json
fi

if ! command -v openapi-codegen >/dev/null; then
    if [[ -z "$GITHUB_WORKFLOW" ]]; then
        echo openapi-codegen is not installed. install using
        echo "cargo install --git https://github.com/svix/openapi-codegen --rev $OPENAPI_GIT_REV --locked"
        exit 1
    else
        cargo install --git https://github.com/svix/openapi-codegen --rev $OPENAPI_GIT_REV --locked
    fi
fi

# Print commands we run
set -x

# Rust
openapi-codegen generate \
    --template rust/templates/api_resource.rs.jinja \
    --input-file lib-openapi.json \
    --output-dir rust/src/api
openapi-codegen generate \
    --template rust/templates/component_type.rs.jinja \
    --input-file lib-openapi.json \
    --output-dir rust/src/models
# Remove APIs we may not (yet) want to expose
rm rust/src/api/{environment,health}.rs
