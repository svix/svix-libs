#!/bin/bash

set -exo pipefail

if [ -n "$1" ]; then
    curl "$1" | python -m json.tool > lib-openapi.json
fi

cd $(dirname "$0")
mkdir -p .codegen-tmp
# OpenAPI version has to be overwritten to avoid broken codegen paths in OpenAPI generator.
# Spec version is overwritten to avoid unnecessary diffs on comments. Same for description.
# `additionalProperties: true` is removed because OpenAPI generator can't deal with it.
jq --indent 4 '
    .openapi = "3.0.2" |
    .info.version = "1.1.1" |
    del(.info.description) |
    .components.schemas |= with_entries(
        if .value | has("properties") then
            .value.properties |= with_entries(
                if .value.additionalProperties == true then
                    del(.value.additionalProperties)
                else .
                end
            )
        else .
        end
    )' \
    < lib-openapi.json \
    > .codegen-tmp/openapi.json

# For some languages, write a separate OpenAPI spec file where optional fields
# of non-`Patch` schemas are set to not be nullable, so the codegen doesn't wrap
# the struct fields in double options.
# Rust's serde and Go's encoding/json will respect both in deserialization
# anyways, but this frees users from having to think about the difference
# between absence of the field or setting it to `null` (which is only
# significant for patch request bodies).
jq --indent 4 '.components.schemas |= with_entries(
        if .key | endswith("Patch") then .
        else
            (.value.required // []) as $required |
            if .value | has("properties") then
                .value.properties |= with_entries(
                    if .key | IN($required[]) then .
                    else del(.value.nullable)
                    end
                )
            else .
            end
        end
    )' \
    < .codegen-tmp/openapi.json \
    > .codegen-tmp/openapi-less-null.json

yarn openapi-generator-cli generate -i .codegen-tmp/openapi.json -g typescript -o javascript/src/openapi -c javascript/openapi-generator-config.json --type-mappings=set=Array -t javascript/templates

yarn openapi-generator-cli generate -i .codegen-tmp/openapi.json -g java -o java/lib/generated/openapi -c java/openapi-generator-config.json -t java/templates

yarn openapi-generator-cli generate -i .codegen-tmp/openapi.json -g ruby -o ruby -c ruby/openapi-generator-config.json -t ruby/templates

rm -rf .codegen-tmp
echo Note: Python generation is not executed automatically.
