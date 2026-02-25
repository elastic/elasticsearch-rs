#!/usr/bin/env bash

# ------------------------------------------------------- #
#
# Skeleton for common build entry script for all elastic
# clients. Needs to be adapted to individual client usage.
#
# Must be called: ./.github/make.sh <target> <params>
#
# Version: 1.1.0
#
# Targets:
# ---------------------------
# bumpmatrix <VERSION> : bump stack version in test matrix to version
# codegen  <VERSION>   : generate endpoints
#
# ------------------------------------------------------- #

# ------------------------------------------------------- #
# Bootstrap
# ------------------------------------------------------- #

script_path=$(dirname "$(realpath "$0")")
repo=$(realpath "$script_path/../")

# shellcheck disable=SC1090
CMD=$1
TASK=$1
TASK_ARGS=()
VERSION=$2
STACK_VERSION=$VERSION
set -euo pipefail

product="elastic/elasticsearch-rs"
output_folder=".github/output"
codegen_folder=".github/output"
OUTPUT_DIR="$repo/${output_folder}"
REPO_BINDING="${OUTPUT_DIR}:/sln/${output_folder}"
WORKFLOW="${WORKFLOW-staging}"
mkdir -p "$OUTPUT_DIR"

echo -e "\033[34;1mINFO:\033[0m PRODUCT ${product}\033[0m"
echo -e "\033[34;1mINFO:\033[0m VERSION ${STACK_VERSION}\033[0m"
echo -e "\033[34;1mINFO:\033[0m OUTPUT_DIR ${OUTPUT_DIR}\033[0m"

# ------------------------------------------------------- #
# Parse Command
# ------------------------------------------------------- #

case $CMD in
    bumpmatrix)
        if [ -v $VERSION ]; then
            echo -e "\033[31;1mTARGET: bumpmatrix -> missing version parameter\033[0m"
            exit 1
        fi
        echo -e "\033[36;1mTARGET: bump stack in test matrix to version $VERSION\033[0m"
        TASK=bumpmatrix
        TASK_ARGS=("$VERSION")
        ;;
    codegen)
        VERSION=$(git rev-parse --abbrev-ref HEAD)
        echo -e "\033[36;1mTARGET: codegen API $VERSION\033[0m"
        TASK=codegen
        TASK_ARGS=("$VERSION")
        ;;
    *)
        echo -e "\nUsage:\n\t $CMD is not supported right now\n"
        exit 1
esac


# ------------------------------------------------------- #
# Build Container
# ------------------------------------------------------- #

build_container() {
  echo -e "\033[34;1mINFO: building $product container\033[0m"

  docker build \
    --build-arg BUILDER_UID="$(id -u)" \
    --file $repo/.buildkite/Dockerfile \
    --tag ${product} \
    .
}

# ------------------------------------------------------- #
# Run the Container
# ------------------------------------------------------- #

echo -e "\033[34;1mINFO: running $product container\033[0m"

if [[ "$CMD" == "bumpmatrix" ]]; then
  TEST_CONFIG_FILE=.buildkite/pipeline.yml
  sed -E -i.bak 's/[0-9]+\.[0-9]+\.[0-9]+-SNAPSHOT/'$VERSION'/g' $TEST_CONFIG_FILE
  rm ${TEST_CONFIG_FILE}.bak
  exit 0
fi

if [[ "$CMD" == "codegen" ]]; then
  build_container
  docker run \
     --rm -v $repo:/usr/src/elasticsearch-rs \
     $product \
     /bin/bash -c "STACK_VERSION=$VERSION cargo make generate-api"

  exit 0
fi

echo "Must be called with '.github/make.sh [command]"
exit 1
