set -u

docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i https://raw.githubusercontent.com/traPtitech/traQ/master/docs/v3-api.yaml \
    -g rust \
    -o /local/
