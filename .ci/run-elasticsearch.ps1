# Launch one or more Elasticsearch nodes via the Docker image,
# to form a cluster suitable for running the REST API tests.
#
# Export the ELASTICSEARCH_VERSION variable, eg. 'elasticsearch:8.0.0-SNAPSHOT'.

param(
  [string]
  $NODE_NAME,

  [string]
  $MASTER_NODE_NAME,

  [string]
  $CLUSTER_NAME,

  [int]
  $HTTP_PORT = 9200,

  [string]
  $ELASTIC_PASSWORD = "changeme",

  [string]
  $SSL_CERT = "$($(Split-Path $script:MyInvocation.MyCommand.Path).Replace("\", "/"))/certs/testnode_san.crt",

  [string]
  $SSL_KEY = "$($(Split-Path $script:MyInvocation.MyCommand.Path).Replace("\", "/"))/certs/testnode_san.key",

  [string]
  $SSL_CA = "$($(Split-Path $script:MyInvocation.MyCommand.Path).Replace("\", "/"))/certs/ca.crt",

  [switch]
  $DETACH,

  [switch]
  $CLEANUP,

  [string]
  $NETWORK_NAME

)

trap {
  cleanup
}

$version = $env:ELASTICSEARCH_VERSION

if ($null -eq $version) {
  Write-Error "ERROR: Required environment variable [ELASTICSEARCH_VERSION] not set"
  exit 1
}

$moniker = $version -replace "[^a-zA-Z\d]", "-"
$suffix = "rest-test"

if (!$NODE_NAME) {
  $NODE_NAME = "${moniker}node1"
}

if (!$MASTER_NODE_NAME) {
  $MASTER_NODE_NAME = $NODE_NAME
}

if (!$CLUSTER_NAME) {
  $CLUSTER_NAME = "${moniker}${suffix}"
}

$volume_name = "${NODE_NAME}-${suffix}-data"

if (!$NETWORK_NAME) {
  $NETWORK_NAME= "${moniker}${suffix}"
}

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

function cleanup_volume {
  param(
    [string]
    $Name
  )

  $volume = docker volume ls --quiet --filter name="$Name"
  if ($volume) {
    log "Removing volume $Name"
    docker volume rm "$Name" > $null
  }
}

function container_running {
  param(
    [string]
    $Name
  )

  $container = docker ps --quiet --filter name="$Name"
  return [bool]$container
}

function cleanup_node {
  param(
    [string]
    $Name
  )

  $container = docker ps --quiet --filter name="$Name"
  if ($container) {
    log "Removing container $Name"
    docker container rm --force --volumes "$Name" > $null
    cleanup_volume "$Name-${suffix}-data" > $null
  }
}

function get_network {
  param(
    [string]
    $Name
  )

  return docker network ls --quiet --filter name="$Name"
}

function cleanup_network {
  param(
    [string]
    $Name
  )

  $network = get_network $Name
  if ($network) {
    log "Removing network $Name"
    docker network rm "$Name" > $null
  }
}

function cleanup {
  param(
    [switch]
    $Cleanup
  )

  if ((-not $DETACH) -or $Cleanup) {
    log "Clean the node and volume on startup (1) OR on exit if not detached"
    cleanup_node $NODE_NAME
  }
  if (-not $DETACH) {
    log "Clean the network if not detached (start and exit)"
    cleanup_network $NETWORK_NAME
  }
}

if ($CLEANUP) {
  #trap - EXIT
  $network = get_network $NETWORK_NAME

  if (-not $network) {
    log "$NETWORK_NAME is already deleted"
    exit 0
  }

  $containers = docker network inspect --format '{{ range $key, $value := .Containers }}{{ println .Name}}{{ end }}' $NETWORK_NAME
  foreach($container in $containers) {
    cleanup_node $container
  }

  cleanup_network $NETWORK_NAME
  log "Cleaned up and exiting" -Level Success
  exit 0
}

log "Making sure previous run leftover infrastructure is removed"
cleanup -Cleanup
log "Creating network $NETWORK_NAME if it does not exist already"

docker network inspect $NETWORK_NAME 2>&1>$null
if ($LASTEXITCODE -ne 0) {
  docker network create "$NETWORK_NAME" > $null
}

$environment = @(
"--env", "node.name=`"$NODE_NAME`"",
"--env", "cluster.name=`"$CLUSTER_NAME`"",
"--env", "cluster.initial_master_nodes=`"$MASTER_NODE_NAME`"",
"--env", "discovery.seed_hosts=`"$MASTER_NODE_NAME`"",
"--env", "cluster.routing.allocation.disk.threshold_enabled=false",
"--env", "bootstrap.memory_lock=true",
"--env", "node.attr.testattr=test",
"--env", "path.repo=/tmp",
"--env", "repositories.url.allowed_urls=http://snapshot.test*"
)

$volumes = @(
"--volume", "${volume_name}:/usr/share/elasticsearch/data"
)

if (-not ($version -match "oss")) {
  $environment += @(
  "--env", "ELASTIC_PASSWORD=`"$ELASTIC_PASSWORD`"",
  "--env", "xpack.license.self_generated.type=trial",
  "--env", "xpack.security.enabled=true",
  "--env", "xpack.security.http.ssl.enabled=true",
  "--env", "xpack.security.http.ssl.verification_mode=certificate",
  "--env", "xpack.security.http.ssl.key=certs/testnode.key",
  "--env", "xpack.security.http.ssl.certificate=certs/testnode.crt",
  "--env", "xpack.security.http.ssl.certificate_authorities=certs/ca.crt",
  "--env", "xpack.security.transport.ssl.enabled=true",
  "--env", "xpack.security.transport.ssl.key=certs/testnode.key",
  "--env", "xpack.security.transport.ssl.certificate=certs/testnode.crt",
  "--env", "xpack.security.transport.ssl.certificate_authorities=certs/ca.crt"
  )

  $volumes += @(
  "--volume", "`"${SSL_CERT}:/usr/share/elasticsearch/config/certs/testnode.crt`"",
  "--volume", "`"${SSL_KEY}:/usr/share/elasticsearch/config/certs/testnode.key`"",
  "--volume", "`"${SSL_CA}:/usr/share/elasticsearch/config/certs/ca.crt`""
  )
}

$url="http://$NODE_NAME"
if (-not ($version -match "oss")) {
  $url="https://elastic:$ELASTIC_PASSWORD@$NODE_NAME"
}

$cert_validation_flags = "--insecure"
if ($NODE_NAME -eq "instance") {
  $cert_validation_flags = "--cacert /usr/share/elasticsearch/config/certs/ca.pem --resolve ${NODE_NAME}:443:127.0.0.1"
}

log "Starting container $NODE_NAME"

if ($DETACH) {
  $d = "true"
} else {
  $d = "false"
}

docker run `
  --name "`"$NODE_NAME`"" `
  --network "`"$NETWORK_NAME`"" `
  --env ES_JAVA_OPTS=-"`"Xms1g -Xmx1g`"" `
  $environment `
  $volumes `
  --publish ${HTTP_PORT}:9200 `
  --ulimit nofile=65536:65536 `
  --ulimit memlock=-1:-1 `
  --detach="$d" `
  --health-cmd="`"curl $cert_validation_flags --fail ${url}:9200/_cluster/health || exit 1`"" `
  --health-interval=2s `
  --health-retries=20 `
  --health-timeout=2s `
  --rm docker.elastic.co/elasticsearch/"$version" > $null

if ($DETACH) {
  while((!$(container_running -Name $NODE_NAME)) -or ((container_running -Name $NODE_NAME) -and ($(docker inspect -f '{{.State.Health.Status}}' $NODE_NAME) -eq "starting"))) {
    Start-Sleep 2;
    $logs = docker inspect -f '{{json .State.Health.Log}}' $NODE_NAME | ConvertFrom-Json
    if ($logs) {
      $lastLog = $logs[$logs.Length-1]
      Write-Output $lastLog.Output
    } else {
      Write-Output "No logs from docker inspect"
    }
    log "waiting for node $NODE_NAME to be up"
  }

  # Always show the node getting started logs, this is very useful both on CI as well as while developing
  if (container_running -Name $NODE_NAME) {
    docker logs $NODE_NAME
  }

  if (!$(container_running -Name $NODE_NAME) -or ("$(docker inspect -f '{{.State.Health.Status}}' ${NODE_NAME})" -ne "healthy")) {
    cleanup -Cleanup
    log "Failed to start $version in detached mode beyond health checks" -Level Error
    log "dumped the docker log before shutting the node down" -Level Error
    exit 1
  } else {
    log "Detached and healthy: $NODE_NAME on docker network: $NETWORK_NAME" -Level Success
    $es_host = $url
    if (!$es_host) {
      $es_host = $NODE_NAME
    }
    if (!$es_host) {
      $es_host = "localhost"
    }

    log "Running on: ${es_host}:${HTTP_PORT}" -Level Success

    # set the environment variable for running locally
    $localUri = New-Object -TypeName System.UriBuilder -ArgumentList "${es_host}:${HTTP_PORT}"
    $localUri.Host = "localhost"
    $env:ES_TEST_SERVER = $localUri.ToString();
    exit 0
  }
}