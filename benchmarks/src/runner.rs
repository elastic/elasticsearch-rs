/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
use crate::{
    record::{Benchmark, Event, Http, HttpResponse, Record},
    Action, Config, Error, Results, Stats, CLIENT_BENCHMARK_CATEGORY,
};
use chrono::Utc;
use elasticsearch::{BulkOperation, BulkParts};
use once_cell::sync::Lazy;
use serde_json::Value;
use std::{collections::BTreeMap, time::Instant};
use tokio::runtime::Runtime;

static STATS_INDEX: Lazy<String> =
    Lazy::new(|| format!("metrics-intake-{}", Utc::now().format("%Y-%m")));

pub(crate) struct Runner<'a> {
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
