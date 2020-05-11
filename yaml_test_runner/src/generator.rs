use inflector::Inflector;
use quote::{ToTokens, Tokens};

use crate::step::*;
use api_generator::generator::Api;
use regex::Regex;
use semver::Version;
use std::collections::HashSet;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Component, PathBuf};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug, PartialEq)]
pub enum TestSuite {
    Oss,
    XPack,
}

/// The components of a test file, constructed from a yaml file
struct YamlTests<'a> {
    version: &'a Version,
    suite: TestSuite,
    directives: HashSet<String>,
    setup: Option<TestFn>,
    teardown: Option<TestFn>,
    tests: Vec<TestFn>,
}

impl<'a> YamlTests<'a> {
    pub fn new(version: &'a semver::Version, suite: TestSuite, len: usize) -> Self {
        Self {
            version,
            suite,
            directives: HashSet::with_capacity(len),
            setup: None,
            teardown: None,
            tests: Vec::with_capacity(len),
        }
    }

    /// Collects the use directives required for all steps in
    /// the test
    fn use_directives_from_steps(steps: &[Step]) -> Vec<String> {
        steps
            .iter()
            .filter_map(Step::r#do)
            .filter_map(|d| d.namespace())
            .map(|s| s.to_string())
            .collect()
    }

    pub fn add_setup(&mut self, setup: TestFn) -> &mut Self {
        let directives = Self::use_directives_from_steps(&setup.steps);
        for directive in directives {
            self.directives.insert(directive);
        }

        self.setup = Some(setup);
        self
    }

    pub fn add_teardown(&mut self, teardown: TestFn) -> &mut Self {
        let directives = Self::use_directives_from_steps(&teardown.steps);
        for directive in directives {
            self.directives.insert(directive);
        }

        self.teardown = Some(teardown);
        self
    }

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
            TestSuite::Oss => quote!(client::general_oss_setup(&client).await?;),
            TestSuite::XPack => quote!(client::general_xpack_setup(&client).await?;),
        };

        let tests: Vec<Tokens> = self.fn_impls(general_setup_call, setup_call, teardown_call);

        let directives: Vec<Tokens> = self
            .directives
            .iter()
            .map(|n| {
                let ident = syn::Ident::from(n.as_str());
                quote!(use elasticsearch::#ident::*;)
            })
            .collect();

        quote! {
            #[allow(unused_imports, unused_variables)]
            #[cfg(test)]
            pub mod tests {
                use elasticsearch::*;
                use elasticsearch::http::{
                    headers::{HeaderName, HeaderValue},
                    request::JsonBody
                };
                use elasticsearch::params::*;
                #(#directives)*
                use regex;
                use serde_json::Value;
                use crate::client;
                use crate::util;

                #setup_fn
                #teardown_fn
                #(#tests)*
            }
        }
    }

    /// some function descriptions are the same in YAML tests, which would result in
    /// duplicate generated test function names. Deduplicate by appending incrementing number
    fn unique_fn_name(name: &str, seen_method_names: &mut HashSet<String>) -> syn::Ident {
        let mut fn_name = name.to_string();
        while !seen_method_names.insert(fn_name.clone()) {
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
        syn::Ident::from(fn_name)
    }

    fn read_response(read_response: bool, is_body_expr: bool, tokens: &mut Tokens) -> bool {
        if !read_response {
            if is_body_expr {
                tokens.append(quote! {
                    let string_response_body = response.text().await?;
                });
            } else {
                tokens.append(quote! {
                    let response_body = response.json::<Value>().await?;
                });
            }
        }

        true
    }

    fn fn_impls(
        &self,
        general_setup_call: Tokens,
        setup_call: Option<Tokens>,
        teardown_call: Option<Tokens>,
    ) -> Vec<Tokens> {
        let mut seen_method_names = HashSet::new();

        self.tests
            .iter()
            .map(|test_fn| {
                let fn_name =
                    Self::unique_fn_name(test_fn.fn_name().as_ref(), &mut seen_method_names);

                let mut body = Tokens::new();
                let mut skip = Option::<&Skip>::None;
                let mut read_response = false;

                for step in &test_fn.steps {
                    match step {
                        Step::Skip(s) => {
                            skip = if s.matches(self.version) {
                                let reason = match s.reason.as_ref() {
                                    Some(s) => s.to_string(),
                                    None => String::new(),
                                };
                                info!(
                                    "Skipping test because skip version '{}' are met. {}",
                                    s.version.as_ref().unwrap(),
                                    reason
                                );
                                Some(s)
                            } else {
                                None
                            }
                        }
                        Step::Do(d) => {
                            read_response = d.to_tokens(false, &mut body);
                        }
                        Step::Match(m) => {
                            read_response = Self::read_response(
                                read_response,
                                m.is_body_expr(&m.expr),
                                &mut body,
                            );
                            m.to_tokens(&mut body);
                        }
                        Step::Set(s) => {
                            // TODO: is "set" ever is_body_expr?
                            read_response = Self::read_response(read_response, false, &mut body);
                            s.to_tokens(&mut body);
                        }
                        Step::Length(l) => {
                            read_response = Self::read_response(
                                read_response,
                                l.is_body_expr(&l.expr),
                                &mut body,
                            );
                            l.to_tokens(&mut body);
                        }
                    }
                }

                // TODO: surface this some other way, other than returning empty tokens
                match skip {
                    Some(_) => Tokens::new(),
                    None => quote! {
                        #[tokio::test]
                        async fn #fn_name() -> Result<(), failure::Error> {
                            let client = client::create();
                            #general_setup_call
                            #setup_call
                            #body
                            #teardown_call
                            Ok(())
                        }
                    },
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

    pub fn fn_name(&self) -> String {
        self.name.replace(" ", "_").to_lowercase().to_snake_case()
    }
}

pub fn generate_tests_from_yaml(
    api: &Api,
    suite: &TestSuite,
    version: &semver::Version,
    base_download_dir: &PathBuf,
    download_dir: &PathBuf,
    generated_dir: &PathBuf,
) -> Result<(), failure::Error> {
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
                    let extension = path.extension().unwrap_or("".as_ref());
                    if extension != "yml" && extension != "yaml" {
                        continue;
                    }

                    let relative_path = path.strip_prefix(&base_download_dir)?;
                    let test_suite = {
                        let mut components = relative_path.components();
                        let mut top_dir = "".to_string();
                        while let Some(c) = components.next() {
                            if c != Component::RootDir {
                                top_dir = c.as_os_str().to_string_lossy().into_owned();
                                break;
                            }
                        }

                        match top_dir.as_str() {
                            "oss" => TestSuite::Oss,
                            "xpack" => TestSuite::XPack,
                            _ => panic!("Unknown test suite"),
                        }
                    };

                    if &test_suite != suite {
                        info!(
                            "skipping {:?}. compiling tests for {:?}",
                            relative_path, suite
                        );
                        continue;
                    }

                    let yaml = fs::read_to_string(&entry.path()).unwrap();

                    // a yaml test can contain multiple yaml docs
                    let result = YamlLoader::load_from_str(&yaml);
                    if result.is_err() {
                        info!(
                            "skipping {:?}. cannot read as Yaml: {}",
                            relative_path,
                            result.err().unwrap().to_string()
                        );
                        continue;
                    }

                    let docs = result.unwrap();
                    let mut test = YamlTests::new(version, test_suite, docs.len());

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
                                        &entry.path(),
                                        &k,
                                        &v,
                                    )))
                                }
                            }
                        })
                        .collect();

                    //if there has been an Err in any step of the yaml test file, don't create a test for it
                    match ok_or_accumulate(&results, 1) {
                        Ok(_) => write_test_file(test, &path, base_download_dir, generated_dir)?,
                        Err(e) => println!(
                            "Error creating test file for {:?}. skipping:\n{}",
                            &entry.path(),
                            e
                        ),
                    }
                }
            }
        }
    }

    write_mod_files(&generated_dir)?;

    Ok(())
}

/// Writes a mod.rs file in each generated directory
fn write_mod_files(generated_dir: &PathBuf) -> Result<(), failure::Error> {
    if !generated_dir.exists() {
        fs::create_dir(generated_dir)?;
    }

    let paths = fs::read_dir(generated_dir)?;
    let mut mods = vec![];
    for path in paths {
        if let Ok(entry) = path {
            let file_type = entry.file_type().unwrap();
            let path = entry.path();
            let name = path.file_stem().unwrap().to_string_lossy();
            if name.into_owned() != "mod" {
                mods.push(format!(
                    "pub mod {};",
                    path.file_stem().unwrap().to_string_lossy()
                ));
            }

            if file_type.is_dir() {
                write_mod_files(&entry.path())?;
            }
        }
    }

    let mut path = generated_dir.clone();
    path.push("mod.rs");
    let mut file = File::create(&path)?;
    let generated_mods: String = mods.join("\n");
    file.write_all(generated_mods.as_bytes())?;
    Ok(())
}

fn write_test_file(
    test: YamlTests,
    path: &PathBuf,
    base_download_dir: &PathBuf,
    generated_dir: &PathBuf,
) -> Result<(), failure::Error> {
    let path = {
        let mut relative = path.strip_prefix(&base_download_dir)?.to_path_buf();
        relative.set_extension("");
        // directories and files will form the module names so ensure they're valid module names
        let clean: String = relative
            .to_string_lossy()
            .into_owned()
            .replace(".", "_")
            .replace("-", "_");
        relative = PathBuf::from(clean);

        let mut path = generated_dir.join(relative);
        path.set_extension("rs");
        // modules can't start with a number so prefix with underscore
        path.set_file_name(format!(
            "_{}",
            &path.file_name().unwrap().to_string_lossy().into_owned()
        ));
        path
    };

    fs::create_dir_all(&path.parent().unwrap())?;
    let mut file = File::create(&path)?;
    file.write_all(
        "// -----------------------------------------------
// ███╗   ██╗ ██████╗ ████████╗██╗ ██████╗███████╗
// ████╗  ██║██╔═══██╗╚══██╔══╝██║██╔════╝██╔════╝
// ██╔██╗ ██║██║   ██║   ██║   ██║██║     █████╗
// ██║╚██╗██║██║   ██║   ██║   ██║██║     ██╔══╝
// ██║ ╚████║╚██████╔╝   ██║   ██║╚██████╗███████╗
// ╚═╝  ╚═══╝ ╚═════╝    ╚═╝   ╚═╝ ╚═════╝╚══════╝
// -----------------------------------------------
//
// This file is generated,
// Please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p yaml_test_runner -- --branch <branch> --token <token> --path <rest specs path>
//
// -----------------------------------------------
"
        .as_bytes(),
    )?;

    let tokens = test.build();
    let generated = api_generator::generator::rust_fmt(tokens.to_string())?;
    let mut file = OpenOptions::new().append(true).open(&path)?;
    file.write_all(generated.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}
