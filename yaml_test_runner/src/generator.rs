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
use crate::step::*;
use api_generator::generator::Api;
use inflector::Inflector;
use path_slash::PathExt;
use quote::{ToTokens, Tokens};
use regex::Regex;
use semver::Version;
use serde::Deserialize;
use std::{
    collections::{BTreeMap, HashSet},
    fs,
    fs::{File, OpenOptions},
    io::Write,
    path::{Component, Path, PathBuf},
};
use yaml_rust::{Yaml, YamlLoader};

/// The test suite to compile
#[derive(Debug, PartialEq)]
pub enum TestSuite {
    Free,
    XPack,
}

/// The components of a test file, constructed from a yaml file
struct YamlTests<'a> {
    path: String,
    version: &'a Version,
    skip: &'a GlobalSkip,
    suite: TestSuite,
    directives: HashSet<String>,
    setup: Option<TestFn>,
    teardown: Option<TestFn>,
    tests: Vec<TestFn>,
}

impl<'a> YamlTests<'a> {
    pub fn new(
        path: &'a Path,
        version: &'a semver::Version,
        skip: &'a GlobalSkip,
        suite: TestSuite,
        len: usize,
    ) -> Self {
        let path = path.to_slash_lossy();
        Self {
            path,
            version,
            skip,
            suite,
            directives: HashSet::with_capacity(len),
            setup: None,
            teardown: None,
            tests: Vec::with_capacity(len),
        }
    }

    /// Collects the use directives required for all steps and tests
    fn use_directives_from_steps(steps: &[Step]) -> Vec<String> {
        steps
            .iter()
            .filter_map(Step::r#do)
            .filter_map(|d| d.namespace())
            .map(|s| s.to_string())
            .collect()
    }

    /// Adds a specific setup function
    pub fn add_setup(&mut self, setup: TestFn) -> &mut Self {
        let directives = Self::use_directives_from_steps(&setup.steps);
        for directive in directives {
            self.directives.insert(directive);
        }

        self.setup = Some(setup);
        self
    }

    /// Adds a specific teardown function
    pub fn add_teardown(&mut self, teardown: TestFn) -> &mut Self {
        let directives = Self::use_directives_from_steps(&teardown.steps);
        for directive in directives {
            self.directives.insert(directive);
        }

        self.teardown = Some(teardown);
        self
    }

    /// Adds a test to the collection of tests
    pub fn add_test_fn(&mut self, test_fn: TestFn) -> &mut Self {
        let directives = Self::use_directives_from_steps(&test_fn.steps);
        for directive in directives {
            self.directives.insert(directive);
        }

        self.tests.push(test_fn);
        self
    }

    /// Generates the AST for the Yaml test file
    pub fn build(self) -> Tokens {
        let (setup_fn, setup_call) = Self::generate_fixture(&self.setup);
        let (teardown_fn, teardown_call) = Self::generate_fixture(&self.teardown);
        let general_setup_call = match self.suite {
            TestSuite::Free => quote!(client::general_oss_setup().await?;),
            TestSuite::XPack => quote!(client::general_xpack_setup().await?;),
        };

        let tests = self.fn_impls(general_setup_call, setup_call, teardown_call);

        let directives: Vec<Tokens> = self
            .directives
            .iter()
            .map(|n| {
                let ident = syn::Ident::from(n.as_str());
                quote!(use elasticsearch::#ident::*;)
            })
            .collect();

        quote! {
            #![allow(unused_imports, unused_variables, dead_code)]
            use crate::common::{client, macros, transform};
            use elasticsearch::*;
            use elasticsearch::http::{
                headers::{HeaderName, HeaderValue},
                request::JsonBody,
                Method,
            };
            use elasticsearch::params::*;
            #(#directives)*
            use ::regex;
            use serde_json::{json, Value};

            #setup_fn
            #teardown_fn
            #(#tests)*
        }
    }

    /// Whether to emit code to read the last response, as text and optionally json
    pub fn read_response(read_response: bool, tokens: &mut Tokens) -> bool {
        if !read_response {
            tokens.append(quote! {
                let (method, status_code, text, json) = client::read_response(response).await?;
            });
        }

        true
    }

    /// Whether the test should be skipped
    fn skip_test(&self, name: &str) -> bool {
        if let Some(tests) = self.skip.tests.get(&self.path) {
            tests.iter().any(|n| n == name || n == "*")
        } else {
            false
        }
    }

    fn fn_impls(
        &self,
        general_setup_call: Tokens,
        setup_call: Option<Tokens>,
        teardown_call: Option<Tokens>,
    ) -> Vec<Option<Tokens>> {
        let mut seen_names = HashSet::new();

        self.tests
            .iter()
            .map(|test_fn| {
                let name = test_fn.name();
                let unique_name = test_fn.unique_name(&mut seen_names);
                if self.skip_test(name) {
                    info!(
                        r#"skipping "{}" in {} because it's included in skip.yml"#,
                        name,
                        self.path,
                    );
                    return None;
                }

                let fn_name = syn::Ident::from(unique_name.as_str());
                let mut body = Tokens::new();
                let mut skip : Option<String> = None;
                let mut read_response = false;

                for step in &test_fn.steps {
                    match step {
                        Step::Skip(s) => {
                            skip = if s.skip_version(self.version) {
                                let m = format!(
                                    r#"skipping "{}" in {} because version "{}" is met. {}"#,
                                    name,
                                    &self.path,
                                    s.version(),
                                    s.reason()
                                );
                                Some(m)
                            } else if s.skip_features(&self.skip.features) {
                                let m = format!(
                                    r#"skipping "{}" in {} because it needs features "{:?}" which are currently not implemented"#,
                                    name,
                                    &self.path,
                                    s.features()
                                );
                                Some(m)
                            } else {
                                None
                            }
                        }
                        Step::Do(d) => {
                            read_response = d.to_tokens(false, &mut body);
                        }
                        Step::Match(m) => {
                            read_response = Self::read_response(read_response,&mut body);
                            m.to_tokens(&mut body);
                        }
                        Step::Set(s) => {
                            read_response = Self::read_response(read_response, &mut body);
                            s.to_tokens(&mut body);
                        }
                        Step::Length(l) => {
                            read_response = Self::read_response(read_response,&mut body);
                            l.to_tokens(&mut body);
                        },
                        Step::IsTrue(t) => {
                            read_response = Self::read_response(read_response,&mut body);
                            t.to_tokens(&mut body);
                        },
                        Step::IsFalse(f) => {
                            read_response = Self::read_response(read_response, &mut body);
                            f.to_tokens(&mut body);
                        },
                        Step::Comparison(c) => {
                            read_response = Self::read_response(read_response,&mut body);
                            c.to_tokens(&mut body);
                        },
                        Step::Contains(c) => {
                            read_response = Self::read_response(read_response,&mut body);
                            c.to_tokens(&mut body);
                        },
                        Step::TransformAndSet(t) => {
                            read_response = Self::read_response(read_response,&mut body);
                            t.to_tokens(&mut body);
                        }
                    }
                }

                match skip {
                    Some(s) => {
                        info!("{}", s);
                        None
                    },
                    None => Some(quote! {
                        #[tokio::test]
                        async fn #fn_name() -> Result<(), failure::Error> {
                            let client = client::get();
                            #general_setup_call
                            #setup_call
                            #body
                            #teardown_call
                            Ok(())
                        }
                    }),
                }
            })
            .collect()
    }

    /// Generates the AST for the fixture fn and its invocation
    fn generate_fixture(test_fn: &Option<TestFn>) -> (Option<Tokens>, Option<Tokens>) {
        if let Some(t) = test_fn {
            let ident = syn::Ident::from(t.name.as_str());

            // TODO: collect up the do calls for now. We do also need to handle skip, etc.
            let tokens = t
                .steps
                .iter()
                .filter_map(Step::r#do)
                .map(|d| {
                    let mut tokens = Tokens::new();
                    ToTokens::to_tokens(d, &mut tokens);
                    tokens
                })
                .collect::<Vec<_>>();

            (
                Some(quote! {
                    async fn #ident(client: &Elasticsearch) -> Result<(), failure::Error> {
                        #(#tokens)*
                        Ok(())
                    }
                }),
                Some(quote! { #ident(&client).await?; }),
            )
        } else {
            (None, None)
        }
    }
}

/// A test function
struct TestFn {
    name: String,
    steps: Vec<Step>,
}

impl TestFn {
    pub fn new<S: Into<String>>(name: S, steps: Vec<Step>) -> Self {
        Self {
            name: name.into(),
            steps,
        }
    }

    /// The function name as declared in yaml
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// some function descriptions are the same in YAML tests, which would result in
    /// duplicate generated test function names. Deduplicate by appending incrementing number
    pub fn unique_name(&self, seen_names: &mut HashSet<String>) -> String {
        let mut fn_name = self.name.replace(" ", "_").to_lowercase().to_snake_case();
        while !seen_names.insert(fn_name.clone()) {
            lazy_static! {
                static ref ENDING_DIGITS_REGEX: Regex = Regex::new(r"^(.*?)_(\d*?)$").unwrap();
            }
            if let Some(c) = ENDING_DIGITS_REGEX.captures(&fn_name) {
                let name = c.get(1).unwrap().as_str();
                let n = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
                fn_name = format!("{}_{}", name, n + 1);
            } else {
                fn_name.push_str("_2");
            }
        }
        fn_name
    }
}

/// Items to globally skip
#[derive(Deserialize)]
struct GlobalSkip {
    features: Vec<String>,
    tests: BTreeMap<String, Vec<String>>,
}

pub fn generate_tests_from_yaml(
    api: &Api,
    suite: &TestSuite,
    version: &semver::Version,
    base_download_dir: &PathBuf,
    download_dir: &PathBuf,
    generated_dir: &PathBuf,
) -> Result<(), failure::Error> {
    let skips = serde_yaml::from_str::<GlobalSkip>(include_str!("./../skip.yml"))?;
    let paths = fs::read_dir(download_dir)?;
    for entry in paths {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    generate_tests_from_yaml(
                        api,
                        suite,
                        version,
                        base_download_dir,
                        &entry.path(),
                        generated_dir,
                    )?;
                } else if file_type.is_file() {
                    let path = entry.path();
                    // skip non-yaml files
                    let extension = path.extension().unwrap_or_else(|| "".as_ref());
                    if extension != "yml" && extension != "yaml" {
                        continue;
                    }

                    let relative_path = path.strip_prefix(&base_download_dir)?;
                    let test_suite = {
                        let components = relative_path.components();
                        let mut top_dir = "".to_string();
                        for c in components {
                            if c != Component::RootDir {
                                top_dir = c.as_os_str().to_string_lossy().into_owned();
                                break;
                            }
                        }

                        match top_dir.as_str() {
                            "free" => TestSuite::Free,
                            "xpack" => TestSuite::XPack,
                            _ => panic!("Unknown test suite {:?}", path),
                        }
                    };

                    if &test_suite != suite {
                        // Belongs to another test suite
                        continue;
                    }

                    let yaml = fs::read_to_string(&entry.path()).unwrap();

                    // a yaml test can contain multiple yaml docs, so use yaml_rust to parse
                    let result = YamlLoader::load_from_str(&yaml);
                    if result.is_err() {
                        error!(
                            "skipping {}. cannot read as Yaml struct: {}",
                            relative_path.to_slash_lossy(),
                            result.err().unwrap().to_string()
                        );
                        continue;
                    }

                    let docs = result.unwrap();
                    let mut test =
                        YamlTests::new(relative_path, version, &skips, test_suite, docs.len());

                    let results : Vec<Result<(), failure::Error>> = docs
                        .iter()
                        .map(|doc| {
                            let hash = doc
                                .as_hash()
                                .ok_or_else(|| failure::err_msg(format!(
                                    "expected hash but found {:?}",
                                    &doc
                                )))?;

                            let (key, value) = hash.iter().next().unwrap();
                            match (key, value) {
                                (Yaml::String(name), Yaml::Array(steps)) => {
                                    let steps = parse_steps(api, steps)?;
                                    let test_fn = TestFn::new(name, steps);
                                    match name.as_str() {
                                        "setup" => test.add_setup(test_fn),
                                        "teardown" => test.add_teardown(test_fn),
                                        _ => test.add_test_fn(test_fn),
                                    };
                                    Ok(())
                                }
                                (k, v) => {
                                    Err(failure::err_msg(format!(
                                        "expected string key and array value in {:?}, but found {:?} and {:?}",
                                        relative_path,
                                        &k,
                                        &v,
                                    )))
                                }
                            }
                        })
                        .collect();

                    //if there has been an Err in any step of the yaml test file, don't create a test for it
                    match ok_or_accumulate(&results) {
                        Ok(_) => write_test_file(test, relative_path, generated_dir)?,
                        Err(e) => {
                            info!("skipping {} because {}", relative_path.to_slash_lossy(), e)
                        }
                    }
                }
            }
        }
    }

    write_mod_files(&generated_dir, true)?;

    Ok(())
}

/// Writes a mod.rs file in each generated directory
fn write_mod_files(generated_dir: &PathBuf, toplevel: bool) -> Result<(), failure::Error> {
    if !generated_dir.exists() {
        fs::create_dir(generated_dir)?;
    }

    let paths = fs::read_dir(generated_dir)?;
    let mut mods = vec![];
    for path in paths {
        if let Ok(entry) = path {
            let path = entry.path();
            let name = path.file_stem().unwrap().to_string_lossy();

            if name != "mod" {
                mods.push(format!(
                    "pub mod {};",
                    path.file_stem().unwrap().to_string_lossy()
                ));
            }

            if path.is_dir() && !(toplevel && name == "common") {
                write_mod_files(&entry.path(), false)?;
            }
        }
    }

    // Make sure we have a stable output
    mods.sort();

    if toplevel {
        // The "common" module must appear first so that its macros are parsed before the
        // compiler visits other modules, otherwise we'll have "macro not found" errors.
        mods.retain(|name| name != "pub mod common;");
        mods.insert(0, "#[macro_use]".into());
        mods.insert(1, "pub mod common;".into());
        mods.insert(2, "".into());
    }

    let mut path = generated_dir.clone();
    path.push("mod.rs");
    let mut file = File::create(&path)?;
    let generated_mods: String = mods.join("\n");
    file.write_all(generated_mods.as_bytes())?;
    Ok(())
}

fn test_file_path(relative_path: &Path) -> Result<PathBuf, failure::Error> {
    let mut relative = relative_path.to_path_buf();
    relative.set_extension("");
    // directories and files will form the module names so ensure they're valid module names
    let clean: String = relative
        .to_string_lossy()
        .replace(".", "_")
        .replace("-", "_");

    relative = PathBuf::from(clean);

    let file_name = relative.file_name().unwrap().to_string_lossy().into_owned();
    // modules can't start with a number so prefix with underscore
    if file_name.starts_with(char::is_numeric) {
        relative.set_file_name(format!("_{}", file_name));
    }

    Ok(relative)
}

fn write_test_file(
    test: YamlTests,
    relative_path: &Path,
    generated_dir: &PathBuf,
) -> Result<(), failure::Error> {
    if test.skip_test("*") {
        info!(
            r#"skipping all tests in {} because it's included in skip.yml"#,
            test.path,
        );
        return Ok(());
    }

    let mut path = test_file_path(relative_path)?;
    path = generated_dir.join(path);
    path.set_extension("rs");

    fs::create_dir_all(&path.parent().unwrap())?;
    let mut file = File::create(&path)?;
    file.write_all(
        r#"/*
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
// -----------------------------------------------
// This file is generated, please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p yaml_test_runner -- --branch <branch> --token <token> --path <rest specs path>
// -----------------------------------------------
"#
        .as_bytes(),
    )?;

    let tokens = test.build();
    let generated = tokens.to_string();
    let mut file = OpenOptions::new().append(true).open(&path)?;
    file.write_all(generated.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}
