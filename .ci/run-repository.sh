#!/usr/bin/env bash
# parameters are available to this script


# STACK_VERSION -- version e.g Major.Minor.Patch(-Prelease)
# TEST_SUITE -- which test suite to run: oss or xpack
# ELASTICSEARCH_URL -- The url at which elasticsearch is reachable, a default is composed based on STACK_VERSION and TEST_SUITE
# BRANCH -- Elasticsearch branch from which to pull the YAML test files
# RUST_TOOLCHAIN -- Rust toolchain version to compile and run tests
# TOKEN -- GitHub access token used to download the YAML files from the GitHub API
script_path=$(dirname $(realpath -s $0))
source $script_path/functions/imports.sh
set -euo pipefail

RUST_TOOLCHAIN=${RUST_TOOLCHAIN-nightly-2020-06-09}
BRANCH=${BRANCH-master}
TOKEN=${TOKEN-}
ELASTICSEARCH_URL=${ELASTICSEARCH_URL-"$elasticsearch_url"}
elasticsearch_container=${elasticsearch_container-}

echo -e "\033[34;1mINFO:\033[0m VERSION ${STACK_VERSION}\033[0m"
echo -e "\033[34;1mINFO:\033[0m TEST_SUITE ${TEST_SUITE}\033[0m"
echo -e "\033[34;1mINFO:\033[0m URL ${ELASTICSEARCH_URL}\033[0m"
echo -e "\033[34;1mINFO:\033[0m CONTAINER ${elasticsearch_container}\033[0m"
echo -e "\033[34;1mINFO:\033[0m BRANCH ${BRANCH}\033[0m"
echo -e "\033[34;1mINFO:\033[0m RUST_TOOLCHAIN ${RUST_TOOLCHAIN}\033[0m"

echo -e "\033[1m>>>>> Build [elastic/elasticsearch-rs container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>\033[0m"

docker build --build-arg RUST_TOOLCHAIN="${RUST_TOOLCHAIN}" --file .ci/DockerFile --tag elastic/elasticsearch-rs .

echo -e "\033[1m>>>>> Run [elastic/elasticsearch-rs container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>\033[0m"

# TODO: output junit.xml file to here
mkdir test_results

repo=$(realpath $(dirname $(realpath -s $0))/../)

docker run \
  --network=${network_name} \
  --env "ES_TEST_SERVER=${ELASTICSEARCH_URL}" \
  --env "ELASTICSEARCH_VERSION=${elasticsearch_container}" \
  --env "TOKEN=${TOKEN}" \
  --name test-runner \
  --volume ${repo}/test_results:/usr/src/elasticsearch-rs/test_results \
  --rm \
  elastic/elasticsearch-rs \
  /bin/bash -c \
  "cargo run -p yaml_test_runner -- --branch \"${BRANCH}\" --path \"/usr/src/elasticsearch-rs/api_generator/rest_specs\"; cargo test -p yaml_test_runner -- --test-threads=1"

