#!/bin/sh

set -ex

if [ ! -z "$1" ]; then
    curl $1 | python -m json.tool > openapi.json
fi

yarn openapi-generator-cli generate -i openapi.json -g python -o python/ -c python/openapi-generator-config.json

yarn openapi-generator-cli generate -i openapi.json -g typescript -o javascript/src/openapi -c javascript/openapi-generator-config.json

yarn openapi-generator-cli generate -i openapi.json -g go -o go/internal/openapi -c go/openapi-generator-config.json