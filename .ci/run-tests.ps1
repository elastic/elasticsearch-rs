param (
    [string]
    [Parameter(Mandatory = $true)]
    $ELASTICSEARCH_VERSION,

    [string]
    [ValidateSet("oss", "xpack")]
    $TEST_SUITE = "oss",

    # TODO: move to stable once elasticsearch-rs compiles on stable
    [string]
    $RUST_VERSION = "nightly",

    [string]
    [ValidateSet("1", "full")]
    $RUST_BACKTRACE
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

function log {
    [CmdletBinding()]
    param(
        [string]
        [Parameter(ValueFromPipeline = $true, Mandatory = $true)]
        $Message,

        [string]
        [ValidateSet("Success", "Info", "Error")]
        $Level
    )

    $ESC = [char]27

    if (!$Level) {
        $Level = "Info"
    }

    switch ($Level) {
        "Success" { $levelMessage = "[32;1mSUCCESS:" }
        "Info" { $levelMessage = "[34;1mINFO:" }
        "Error" { $levelMessage = "[31;1mERROR:" }
    }

    Write-Output "$ESC$levelMessage$ESC[0m $message$ESC[0m"
}

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
      log "run-tests" -Level Success
      exit 0
    } else {
      log "failure during run-tests" -Level Error
      exit $status
    }
}

Write-Output ">>>>> Start [$env:ELASTICSEARCH_VERSION container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>"

$network = "elasticsearch"

$runParams = @{
  NODE_NAME= $NODE_NAME
  NETWORK_NAME = $network
  DETACH = $true  
}

./.ci/run-elasticsearch.ps1 @runParams

Write-Output ">>>>> Build [elastic/elasticsearch-rs container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>"
docker build --file .ci/DockerFile --tag elastic/elasticsearch-rs .
if ($LASTEXITCODE) {
    log "Problem building docker container" -Level Error
    exit 1
}
Write-Output ">>>>> Run [elastic/elasticsearch-rs container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>"

$environment = @(
  "--env", "ES_TEST_SERVER=${elasticsearch_url}",
  "--env", "RUST_VERSION=$RUST_VERSION"
)

if ($RUST_BACKTRACE) {
    $environment += @(
        "--env", "RUST_BACKTRACE=$RUST_BACKTRACE"
    )
}

docker run `
  --network=$network `
  $environment `
  --name elasticsearch-rs `
  --rm `
elastic/elasticsearch-rs `
cargo test

if ($LASTEXITCODE) {
    docker rm elasticsearch-rs
}

cleanup