#!/usr/bin/env bash
# parameters are available to this script

# common build entry script for all elasticsearch clients

# ./.ci/make.sh assemble VERSION

script_path=$(dirname "$(realpath -s "$0")")
repo=$(realpath "$script_path/../")

# shellcheck disable=SC1090
CMD=$1
TASK=$1
VERSION=$2
STACK_VERSION=$VERSION
set -euo pipefail

output_folder=".ci/output"
OUTPUT_DIR="$repo/${output_folder}"
mkdir -p "$OUTPUT_DIR"

RUST_TOOLCHAIN=${RUST_TOOLCHAIN-latest}

echo -e "\033[34;1mINFO:\033[0m VERSION ${STACK_VERSION}\033[0m"
echo -e "\033[34;1mINFO:\033[0m OUTPUT_DIR ${OUTPUT_DIR}\033[0m"
echo -e "\033[34;1mINFO:\033[0m RUST_TOOLCHAIN ${RUST_TOOLCHAIN}\033[0m"

echo -e "\033[1m>>>>> Build [elastic/elasticsearch-rs container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>\033[0m"

docker build --build-arg RUST_TOOLCHAIN="${RUST_TOOLCHAIN}" --file .ci/DockerFile --tag elastic/elasticsearch-rs .

echo -e "\033[1m>>>>> Run [elastic/elasticsearch-rs container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>\033[0m"

case $CMD in
    assemble)
        TASK=release
        ;;
    *)
        echo -e "\nUsage:\n\t $CMD is not supported right now\n"
        exit 1
esac

docker run \
  --env "CI=true" \
  --name make-elasticsearch-rs \
  --volume "${OUTPUT_DIR}:/usr/src/elasticsearch-rs/${output_folder}" \
  --volume ${repo}/test_results:/usr/src/elasticsearch-rs/test_results \
  --rm \
  elastic/elasticsearch-rs \
  echo 'something calling automation here using $TASK and $VERSION producing output in $OUTPUT_DIR'

