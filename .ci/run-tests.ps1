param (
    [string]
    [Parameter(Mandatory = $true)]
    $ELASTICSEARCH_VERSION,

    [string]
    [ValidateSet("oss", "xpack")]
    $TEST_SUITE = "oss",

    # TODO: move to stable once elasticsearch-rs compiles on stable
    [string]
    $RUST_VERSION = "nightly"
)

trap {
  cleanup
}

$NODE_NAME = "es1"
$elasticsearch_image= "elasticsearch"
$elasticsearch_url = "https://elastic:changeme@${NODE_NAME}:9200"

if ($TEST_SUITE -ne "xpack") {
  $elasticsearch_image= "elasticsearch-${TEST_SUITE}"
  $elasticsearch_url = "http://${NODE_NAME}:9200"
}

# set for run-elasticsearch.ps1
$env:ELASTICSEARCH_VERSION = "${elasticsearch_image}:$ELASTICSEARCH_VERSION"

function cleanup {
    $status=$?

    $runParams = @{
      NODE_NAME= $NODE_NAME
      NETWORK_NAME = "elasticsearch"
      CLEANUP = $true  
    }

    ./.ci/run-elasticsearch.ps1 @runParams

    # Report status and exit
    if ($status -eq 0) {
      Write-Output "SUCCESS run-tests"
      exit 0
    } else {
      Write-Output "FAILURE during run-tests"
      exit $status
    }
}


Write-Output ">>>>> Start [$env:ELASTICSEARCH_VERSION container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>"

$runParams = @{
  NODE_NAME= "$NODE_NAME"
  NETWORK_NAME = "elasticsearch"
  DETACH = $true  
}

./.ci/run-elasticsearch.ps1 @runParams

Write-Output ">>>>> Build [elastic/elasticsearch-rs container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>"

docker build --file .ci/DockerFile --tag elastic/elasticsearch-rs .

Write-Output ">>>>> Run [elastic/elasticsearch-rs container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>"

docker run `
  --network=elasticsearch `
  --env "ES_TEST_SERVER=${elasticsearch_url}" `
  --env "RUST_VERSION=$RUST_VERSION" `
  --name elasticsearch-rs `
  --rm `
elastic/elasticsearch-rs `
cargo test

cleanup