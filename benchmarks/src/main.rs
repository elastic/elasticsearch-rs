use chrono::{DateTime, Duration, Utc};
use elasticsearch::{http::response::Response, BulkOperation, BulkParts, Elasticsearch};
use std::{error, fmt, fs};
use tokio::runtime::Runtime;
#[macro_use]
extern crate serde_json;
extern crate rustc_version_runtime;
extern crate sys_info;
use crate::{
    actions::{get::get, index::index, info::info, ping::ping},
    record::{Benchmark, Event, Git, Http, HttpResponse, Os, Record, Target},
};
use elasticsearch::http::transport::Transport;
use once_cell::sync::Lazy;
use rustc_version_runtime::version;
use serde_json::Value;
use std::{collections::BTreeMap, fs::File, io::BufReader, path::PathBuf, time::Instant};

mod actions;
mod record;

static CLIENT_BENCHMARK_CATEGORY: Lazy<String> =
    Lazy::new(|| std::env::var("CLIENT_BENCHMARK_CATEGORY").unwrap_or_else(|_| "".to_string()));

static FILTER: Lazy<String> =
    Lazy::new(|| std::env::var("FILTER").unwrap_or_else(|_| "".to_string()));

static STATS_INDEX: Lazy<String> =
    Lazy::new(|| format!("metrics-intake-{}", Utc::now().format("%Y-%m")));

fn main() -> Result<(), Error> {
    let rustc_version = version();
    let config = Config::new(rustc_version.to_string())?;
    let benchmarks = Benchmarks::new();
    let mut runtime = Runtime::new().unwrap();

    for operation in benchmarks.operations {
        if !FILTER.is_empty() && !FILTER.contains(&operation.action) {
            continue;
        }

        let mut runner = Runner::new(&config, &operation);

        match runner.run(&mut runtime) {
            Ok(results) => println!(
                "{}, repetitions: {}, mean: {} ns, ops/sec: {}, errors: {}",
                &results.action,
                &results.repetitions,
                &results.mean,
                &results.ops_sec,
                &results.errors.len()
            ),
            Err(e) => println!("{}", e.to_string()),
        }
    }

    Ok(())
}

struct Benchmarks {
    operations: Vec<Action>,
}

impl Benchmarks {
    pub fn new() -> Self {
        Self {
            operations: vec![ping(), index(), get(), info()],
        }
    }
}

pub struct Config {
    build_id: String,
    environment: String,
    target: record::Target,
    runner: record::Runner,
    runner_client: Elasticsearch,
    report_client: Elasticsearch,
    data_sources: BTreeMap<String, Value>,
}

impl Config {
    pub fn new(rustc_version: String) -> Result<Self, Error> {
        let env_keys = vec![
            "BUILD_ID",
            "DATA_SOURCE",
            "CLIENT_BRANCH",
            "CLIENT_COMMIT",
            "CLIENT_BENCHMARK_ENVIRONMENT",
            "ELASTICSEARCH_TARGET_URL",
            "ELASTICSEARCH_REPORT_URL",
            "TARGET_SERVICE_TYPE",
            "TARGET_SERVICE_NAME",
            "TARGET_SERVICE_VERSION",
            "TARGET_SERVICE_OS_FAMILY",
        ];

        let (vars, errors): (Vec<_>, Vec<_>) = env_keys
            .iter()
            .map(|e| match std::env::var(e) {
                Ok(v) if !v.is_empty() => Ok((e.to_string(), v)),
                Ok(_) => Err(Error::config(format!(
                    "{} environment variable is empty",
                    e
                ))),
                Err(err) => Err(Error::config(format!("{} {}", e, err.to_string()))),
            })
            .partition(Result::is_ok);

        if !errors.is_empty() {
            let errors: Vec<_> = errors
                .into_iter()
                .map(|e| e.unwrap_err().to_string())
                .collect();
            return Err(Error::config(errors.join("\n")));
        }

        let vars = vars
            .into_iter()
            .map(Result::unwrap)
            .collect::<BTreeMap<String, String>>();

        let data_source = PathBuf::from(vars.get("DATA_SOURCE").unwrap());
        if !data_source.exists() {
            return Err(Error::config(format!(
                "Data source at {} does not exist",
                data_source.to_str().unwrap()
            )));
        }

        let mut data_sources = BTreeMap::new();
        let entries = fs::read_dir(&data_source)?;
        for entry in entries {
            if let Ok(e) = entry {
                if let Ok(f) = e.file_type() {
                    if !f.is_dir() {
                        continue;
                    }

                    let reader = {
                        let mut path = e.path().clone();
                        path.push("document.json");
                        let file = File::open(path)?;
                        BufReader::new(file)
                    };
                    let json: Value = serde_json::from_reader(reader)?;

                    data_sources.insert(e.file_name().to_string_lossy().to_string(), json);
                }
            }
        }

        let service = record::Service {
            ty: vars.get("TARGET_SERVICE_TYPE").unwrap().to_string(),
            name: vars.get("TARGET_SERVICE_NAME").unwrap().to_string(),
            version: vars.get("TARGET_SERVICE_VERSION").unwrap().to_string(),
            git: Git {
                commit: vars.get("CLIENT_COMMIT").unwrap().to_string(),
                branch: vars.get("CLIENT_BRANCH").unwrap().to_string(),
            },
        };

        let os = Os {
            family: vars.get("TARGET_SERVICE_OS_FAMILY").unwrap().to_string(),
        };

        Ok(Self {
            build_id: vars.get("BUILD_ID").unwrap().to_string(),
            environment: vars
                .get("CLIENT_BENCHMARK_ENVIRONMENT")
                .unwrap()
                .to_string(),
            target: Target {
                service: service.clone(),
                os: os.clone(),
            },
            runner: record::Runner {
                service,
                runtime: record::Runtime {
                    name: "rust".to_string(),
                    version: rustc_version,
                },
                os,
            },
            runner_client: Elasticsearch::new(
                Transport::single_node(vars.get("ELASTICSEARCH_TARGET_URL").unwrap()).unwrap(),
            ),
            report_client: Elasticsearch::new(
                Transport::single_node(vars.get("ELASTICSEARCH_REPORT_URL").unwrap()).unwrap(),
            ),
            data_sources,
        })
    }

    pub fn runner_client(&self) -> &Elasticsearch {
        &self.runner_client
    }

    pub fn report_client(&self) -> &Elasticsearch {
        &self.report_client
    }

    pub fn environment(&self) -> &str {
        self.environment.as_str()
    }

    pub fn data_sources(&self, key: &str) -> Option<&Value> {
        self.data_sources.get(key)
    }
}

#[derive(Clone)]
struct Stats {
    start: DateTime<Utc>,
    duration: Duration,
    outcome: String,
    status_code: Option<u16>,
}

struct Results {
    action: String,
    repetitions: i32,
    errors: Vec<String>,
    mean: i64,
    ops_sec: f64,
}

struct Runner<'a> {
    config: &'a Config,
    stats: Vec<Stats>,
    action: &'a Action,
}

impl<'a> Runner<'a> {
    pub fn new(config: &'a Config, action: &'a Action) -> Self {
        Self {
            config,
            stats: Vec::new(),
            action,
        }
    }

    pub fn run(&mut self, runtime: &mut Runtime) -> Result<Results, Error> {
        let operations = self.action.operations.unwrap_or_else(|| 1);
        let category = self
            .action
            .category()
            .unwrap_or_else(|| CLIENT_BENCHMARK_CATEGORY.as_ref())
            .to_string();
        let environment = self
            .action
            .environment()
            .unwrap_or_else(|| self.config.environment())
            .to_string();

        let mut errors: Vec<String> = Vec::with_capacity(
            (self.action.warmups + (self.action.repetitions * operations)) as usize,
        );

        if let Some(f) = self.action.setup {
            (f)(self.config, runtime)?;
        }

        for i in 0..self.action.warmups {
            match self.action.measure(i, self.config, runtime) {
                Ok(r) => {
                    if !r.status_code().is_success() {
                        let e = r.error_for_status_code().err().unwrap();
                        errors.push(format!("warmup {}: {}", i, e.to_string()))
                    }
                }
                Err(e) => errors.push(format!("warmup {}: {}", i, e.to_string())),
            }
        }

        for i in 0..self.action.repetitions {
            let start = Utc::now();
            let now = Instant::now();
            let result = self.action.measure(i, self.config, runtime);
            let duration = now.elapsed();
            let mut outcome = String::new();
            let mut status_code: Option<u16> = None;
            match result {
                Ok(r) => {
                    status_code = Some(r.status_code().as_u16());
                    if !r.status_code().is_success() {
                        let e = r.error_for_status_code().err().unwrap();
                        errors.push(format!("run {}: {}", i, e.to_string()));
                        outcome.push_str("failure");
                    } else {
                        outcome.push_str("success");
                    }
                }
                Err(e) => {
                    errors.push(format!("run {}: {}", i, e.to_string()));
                    outcome.push_str("failure");
                }
            }

            self.stats.push(Stats {
                start,
                duration: chrono::Duration::from_std(duration).unwrap(),
                outcome,
                status_code,
            });
        }

        if errors.is_empty() {
            self.save_stats(runtime, operations, category, environment)
                .ok()
                .unwrap();

            let mean = {
                (self
                    .stats
                    .iter()
                    .map(|s| s.duration.num_nanoseconds().unwrap() as f64)
                    .sum::<f64>()
                    / self.stats.len() as f64) as i64
            };

            Ok(Results {
                action: self.action.action.clone(),
                repetitions: self.action.repetitions,
                errors,
                mean,
                ops_sec: { 1e+9f64 / mean as f64 },
            })
        } else {
            Err(Error::run(errors))
        }
    }

    fn save_stats(
        &self,
        runtime: &mut Runtime,
        operations: i32,
        category: String,
        environment: String,
    ) -> Result<(), Error> {
        let chunk_size = 1_000;
        let client = self.config.report_client();
        for chunk in self.stats.chunks(chunk_size) {
            let mut body: Vec<BulkOperation<Record>> = Vec::with_capacity(1_000);

            for stat in chunk {
                body.push(
                    BulkOperation::index(Record {
                        timestamp: stat.start,
                        labels: {
                            let mut map = BTreeMap::new();
                            map.insert("build_id".into(), self.config.build_id.clone());
                            map.insert("client".into(), "elasticsearch-rs".into());
                            map.insert("environment".into(), environment.clone());
                            map
                        },
                        tags: vec!["bench".into(), "elasticsearch-rs".into()],
                        event: Event {
                            action: self.action.action.clone(),
                            duration: stat.duration.num_nanoseconds().unwrap(),
                            outcome: stat.outcome.clone(),
                            dataset: None,
                        },
                        http: Http {
                            response: HttpResponse {
                                status_code: stat.status_code,
                            },
                        },
                        benchmark: Benchmark {
                            build_id: self.config.build_id.clone(),
                            repetitions: self.action.repetitions,
                            operations,
                            runner: self.config.runner.clone(),
                            target: self.config.target.clone(),
                            category: category.clone(),
                            environment: environment.clone(),
                        },
                    })
                    .into(),
                );
            }

            let response = runtime.block_on(async {
                client
                    .bulk(BulkParts::Index(STATS_INDEX.as_str()))
                    .body(body)
                    .send()
                    .await
            });

            match response {
                Ok(r) => {
                    if !r.status_code().is_success() {
                        println!("HTTP {} error saving stats", r.status_code().as_u16());
                    } else {
                        let de = runtime.block_on(async { r.json::<Value>().await });
                        match de {
                            Ok(json) => {
                                if json["errors"].as_bool().unwrap() {
                                    let error_count = json["items"]
                                        .as_array()
                                        .unwrap()
                                        .iter()
                                        .filter(|item| item["index"]["error"].is_object())
                                        .count();

                                    println!("{} errors saving stats", error_count);
                                }
                            }
                            Err(e) => println!("Error saving stats: {}", e.to_string()),
                        }
                    }
                }
                Err(e) => println!("Error saving stats: {}", e.to_string()),
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Error {
    kind: Kind,
}

impl Error {
    pub(crate) fn config(err: impl Into<String>) -> Self {
        Error {
            kind: Kind::Config(err.into()),
        }
    }

    pub(crate) fn run(errs: Vec<String>) -> Self {
        Error {
            kind: Kind::Run(errs),
        }
    }
}

impl From<elasticsearch::Error> for Error {
    fn from(err: elasticsearch::Error) -> Self {
        Error {
            kind: Kind::Response(err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            kind: Kind::IO(err),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error {
            kind: Kind::DataSource(err),
        }
    }
}

#[derive(Debug)]
enum Kind {
    Config(String),
    IO(std::io::Error),
    DataSource(serde_json::Error),
    Response(elasticsearch::Error),
    Run(Vec<String>),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            Kind::Config(_) => None,
            Kind::IO(err) => Some(err),
            Kind::DataSource(err) => Some(err),
            Kind::Response(err) => Some(err),
            Kind::Run(_) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::Config(err) => err.fmt(f),
            Kind::DataSource(err) => err.fmt(f),
            Kind::IO(err) => err.fmt(f),
            Kind::Response(err) => err.fmt(f),
            Kind::Run(errs) => errs.join("\n").fmt(f),
        }
    }
}

pub struct Action {
    action: String,
    category: Option<String>,
    environment: Option<String>,
    warmups: i32,
    repetitions: i32,
    operations: Option<i32>,
    setup: Option<fn(&Config, &mut Runtime) -> Result<Response, elasticsearch::Error>>,
    run: fn(i32, &Config, &mut Runtime) -> Result<Response, elasticsearch::Error>,
}

impl Action {
    pub fn category(&self) -> Option<&str> {
        self.category.as_deref()
    }

    pub fn environment(&self) -> Option<&str> {
        self.environment.as_deref()
    }

    pub fn measure(
        &self,
        i: i32,
        config: &Config,
        runtime: &mut Runtime,
    ) -> Result<Response, elasticsearch::Error> {
        (self.run)(i, config, runtime)
    }
}
