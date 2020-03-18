use inflector::Inflector;
use quote::{Tokens, ToTokens};

use api_generator::generator::{Api, ApiEndpoint, TypeKind};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashSet, BTreeMap};
use std::fmt::Write as FormatWrite;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use yaml_rust::{yaml::Hash, Yaml, YamlEmitter, YamlLoader};
use semver::Version;

/// The components of a test file, constructed from a yaml file
struct YamlTests {
    version: Option<Version>,
    directives: HashSet<String>,
    setup: Option<Vec<Step>>,
    teardown: Option<Vec<Step>>,
    tests: Vec<YamlTestFn>,
}

impl YamlTests {
    pub fn new(version: Option<semver::Version>, len: usize) -> Self {
        Self {
            version,
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
        .filter_map(|d| d.api_call.namespace.as_ref())
        .map(|s| s.to_string())
        .collect()
    }

    pub fn add_setup(&mut self, steps: Vec<Step>) -> &mut Self {
        let directives = Self::use_directives_from_steps(&steps);
        for directive in directives {
            self.directives.insert(directive);
        }

        self.setup = Some(steps);
        self
    }

    pub fn add_teardown(&mut self, steps: Vec<Step>) -> &mut Self {
        let directives = Self::use_directives_from_steps(&steps);
        for directive in directives {
            self.directives.insert(directive);
        }

        self.teardown = Some(steps);
        self
    }

    pub fn add_test_fn(&mut self, test_fn: YamlTestFn) -> &mut Self {
        let directives = Self::use_directives_from_steps(&test_fn.steps);
        for directive in directives {
            self.directives.insert(directive);
        }

        self.tests.push(test_fn);
        self
    }

    pub fn build(self) -> Tokens {
        let (setup_fn, setup_call) = self.setup_impl();
        let (teardown_fn, teardown_call) = self.teardown_impl();
        let tests: Vec<Tokens> = self.fn_impls(setup_call, teardown_call);

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
                use elasticsearch::http::request::JsonBody;
                use elasticsearch::params::*;
                #(#directives)*
                use regex;
                use serde_json::Value;
                use crate::client;

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
                static ref ENDING_DIGITS_REGEX: Regex =
                    Regex::new(r"^(.*?)_(\d*?)$").unwrap();
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

    fn fn_impls(&self, setup_call: Option<Tokens>, teardown_call: Option<Tokens>) -> Vec<Tokens> {
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
                            skip = if let Some(v) = &self.version {
                                if s.matches(v) {
                                    let reason = match s.reason.as_ref() {
                                        Some(s) => s.to_string(),
                                        None => String::new()
                                    };
                                    println!(
                                        "Skipping test because skip version '{}' are met. {}",
                                        s.version.as_ref().unwrap(),
                                        reason
                                    );
                                    Some(s)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        },
                        Step::Do(d) => d.to_tokens(&mut body),
                        Step::Match(m) => {
                            if !read_response {
                                body.append(quote! {
                                    let response_body = response.read_body::<Value>().await?;
                                });
                                read_response = true;
                            }
                            m.to_tokens(&mut body);
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
                            #setup_call
                            #body
                            #teardown_call
                            Ok(())
                        }
                    }
                }
            })
            .collect()
    }

    fn setup_impl(&self) -> (Option<Tokens>, Option<Tokens>) {
        Self::generate_fixture("setup", &self.setup)
    }

    fn teardown_impl(&self) -> (Option<Tokens>, Option<Tokens>) {
        Self::generate_fixture("teardown", &self.teardown)
    }

    /// Generates the AST for the fixture fn and its invocation
    fn generate_fixture(name: &str, steps: &Option<Vec<Step>>) -> (Option<Tokens>, Option<Tokens>) {
        if let Some(s) = steps {
            let ident = syn::Ident::from(name);

            // TODO: collect up the do calls for now. We do also need to handle skip, etc.
            let tokens = s
                .iter()
                .filter_map(Step::r#do)
                .map(|d| {
                    let mut tokens = Tokens::new();
                    d.to_tokens(&mut tokens);
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
struct YamlTestFn {
    name: String,
    steps: Vec<Step>,
}

impl YamlTestFn {
    pub fn new<S: Into<String>>(name: S, steps: Vec<Step>) -> Self {
        Self {
            name: name.into(),
            steps
        }
    }

    pub fn fn_name(&self) -> String {
        self.name.replace(" ", "_").to_lowercase().to_snake_case()
    }
}

/// The components of an API call
struct ApiCall {
    namespace: Option<String>,
    function: syn::Ident,
    parts: Option<Tokens>,
    params: Option<Tokens>,
    body: Option<Tokens>,
    ignore: Option<i64>,
}

impl ToTokens for ApiCall {
    fn to_tokens(&self, tokens: &mut Tokens) {
        let function = &self.function;
        let parts = &self.parts;
        let params = &self.params;
        let body = &self.body;

        tokens.append(quote! {
            let response = client.#function(#parts)
                #params
                #body
                .send()
                .await?;
        });
    }
}

impl ApiCall {
    /// Try to create an API call
    pub fn try_from(
        api: &Api,
        endpoint: &ApiEndpoint,
        hash: &Hash,
    ) -> Result<ApiCall, failure::Error> {
        let mut parts: Vec<(&str, &Yaml)> = vec![];
        let mut params: Vec<(&str, &Yaml)> = vec![];
        let mut body: Option<Tokens> = None;
        let mut ignore: Option<i64> = None;

        // work out what's a URL part and what's a param in the supplied
        // arguments for the API call
        for (k, v) in hash.iter() {
            let key = k.as_str().unwrap();
            if endpoint.params.contains_key(key) || api.common_params.contains_key(key) {
                params.push((key, v));
            } else if key == "body" {
                body = Self::generate_body(endpoint, v);
            } else if key == "ignore" {
                ignore = match v.as_i64() {
                    Some(i) => Some(i),
                    // handle ignore as an array of i64
                    None => v.as_vec().unwrap()[0].as_i64(),
                }
            } else {
                parts.push((key, v));
            }
        }

        let api_call = endpoint.full_name.as_ref().unwrap();
        let parts = Self::generate_parts(api_call, endpoint, &parts)?;
        let params = Self::generate_params(api, endpoint, &params)?;
        let function = syn::Ident::from(api_call.replace(".", "()."));
        let namespace: Option<String> = if api_call.contains('.') {
            let namespaces: Vec<&str> = api_call.splitn(2, '.').collect();
            Some(namespaces[0].to_string())
        } else {
            None
        };

        Ok(ApiCall {
            namespace,
            function,
            parts,
            params,
            body,
            ignore,
        })
    }

    fn generate_params(
        api: &Api,
        endpoint: &ApiEndpoint,
        params: &[(&str, &Yaml)],
    ) -> Result<Option<Tokens>, failure::Error> {
        match params.len() {
            0 => Ok(None),
            _ => {
                let mut tokens = Tokens::new();
                for (n, v) in params {
                    let param_ident =
                        syn::Ident::from(api_generator::generator::code_gen::valid_name(n));

                    let ty = match endpoint.params.get(*n) {
                        Some(t) => Ok(t),
                        None => match api.common_params.get(*n) {
                            Some(t) => Ok(t),
                            None => Err(failure::err_msg(format!("No param found for {}", n))),
                        },
                    }?;

                    let kind = ty.ty;

                    fn create_enum(
                        enum_name: &str,
                        variant: &str,
                        options: &[serde_json::Value],
                    ) -> Result<Tokens, failure::Error> {
                        if !variant.is_empty()
                            && !options.contains(&serde_json::Value::String(variant.to_owned()))
                        {
                            return Err(failure::err_msg(format!(
                                "options {:?} does not contain value {}",
                                &options, variant
                            )));
                        }

                        let e: String = enum_name.to_pascal_case();
                        let enum_name = syn::Ident::from(e.as_str());
                        let variant = if variant.is_empty() {
                            // TODO: Should we simply omit empty Refresh tests?
                            if e == "Refresh" {
                                syn::Ident::from("True")
                            } else if e == "Size" {
                                syn::Ident::from("Unspecified")
                            } else {
                                return Err(failure::err_msg(format!(
                                    "Unhandled empty value for {}",
                                    &e
                                )));
                            }
                        } else {
                            syn::Ident::from(variant.to_pascal_case())
                        };

                        Ok(quote!(#enum_name::#variant))
                    }

                    match v {
                        Yaml::String(ref s) => {
                            match kind {
                                TypeKind::Enum => {
                                    if n == &"expand_wildcards" {
                                        // expand_wildcards might be defined as a comma-separated
                                        // string. e.g.
                                        let idents: Vec<Result<Tokens, failure::Error>> = s
                                            .split(',')
                                            .collect::<Vec<_>>()
                                            .iter()
                                            .map(|e| create_enum(n, e, &ty.options))
                                            .collect();

                                        match ok_or_accumulate(&idents, 0) {
                                            Ok(_) => {
                                                let idents: Vec<Tokens> = idents
                                                    .into_iter()
                                                    .filter_map(Result::ok)
                                                    .collect();

                                                tokens.append(quote! {
                                                    .#param_ident(&[#(#idents),*])
                                                });
                                            }
                                            Err(e) => return Err(failure::err_msg(e)),
                                        }
                                    } else {
                                        let e = create_enum(n, s.as_str(), &ty.options)?;
                                        tokens.append(quote! {
                                            .#param_ident(#e)
                                        });
                                    }
                                }
                                TypeKind::List => {
                                    let values: Vec<&str> = s.split(',').collect();
                                    tokens.append(quote! {
                                        .#param_ident(&[#(#values),*])
                                    })
                                }
                                TypeKind::Boolean => {
                                    let b = s.parse::<bool>()?;
                                    tokens.append(quote! {
                                        .#param_ident(#b)
                                    });
                                }
                                TypeKind::Double => {
                                    let f = s.parse::<f64>()?;
                                    tokens.append(quote! {
                                        .#param_ident(#f)
                                    });
                                }
                                TypeKind::Integer | TypeKind::Number => {
                                    let i = s.parse::<i32>()?;
                                    tokens.append(quote! {
                                        .#param_ident(#i)
                                    });
                                }
                                _ => tokens.append(quote! {
                                    .#param_ident(#s)
                                }),
                            }
                        }
                        Yaml::Boolean(ref b) => match kind {
                            TypeKind::Enum => {
                                let enum_name = syn::Ident::from(n.to_pascal_case());
                                let variant = syn::Ident::from(b.to_string().to_pascal_case());
                                tokens.append(quote! {
                                    .#param_ident(#enum_name::#variant)
                                })
                            }
                            TypeKind::List => {
                                // TODO: _source filter can be true|false|list of strings
                                let s = b.to_string();
                                tokens.append(quote! {
                                    .#param_ident(&[#s])
                                })
                            }
                            _ => {
                                tokens.append(quote! {
                                    .#param_ident(#b)
                                });
                            }
                        },
                        Yaml::Integer(ref i) => match kind {
                            TypeKind::String => {
                                let s = i.to_string();
                                tokens.append(quote! {
                                    .#param_ident(#s)
                                })
                            }
                            TypeKind::Integer => {
                                // yaml-rust parses all as i64
                                let int = *i as i32;
                                tokens.append(quote! {
                                    .#param_ident(#int)
                                });
                            }
                            TypeKind::Float => {
                                // yaml-rust parses all as i64
                                let f = *i as f32;
                                tokens.append(quote! {
                                    .#param_ident(#f)
                                });
                            }
                            TypeKind::Double => {
                                // yaml-rust parses all as i64
                                let f = *i as f64;
                                tokens.append(quote! {
                                    .#param_ident(#f)
                                });
                            }
                            _ => {
                                tokens.append(quote! {
                                    .#param_ident(#i)
                                });
                            }
                        },
                        Yaml::Array(arr) => {
                            // only support param string arrays
                            let result: Vec<&String> = arr
                                .iter()
                                .map(|i| match i {
                                    Yaml::String(s) => Ok(s),
                                    y => Err(failure::err_msg(format!(
                                        "Unsupported array value {:?}",
                                        y
                                    ))),
                                })
                                .filter_map(Result::ok)
                                .collect();

                            if n == &"expand_wildcards" {
                                let result: Vec<Result<Tokens, failure::Error>> = result
                                    .iter()
                                    .map(|s| create_enum(n, s.as_str(), &ty.options))
                                    .collect();

                                match ok_or_accumulate(&result, 0) {
                                    Ok(_) => {
                                        let result: Vec<Tokens> =
                                            result.into_iter().filter_map(Result::ok).collect();

                                        tokens.append(quote! {
                                            .#param_ident(&[#(#result),*])
                                        });
                                    }
                                    Err(e) => return Err(failure::err_msg(e)),
                                }
                            } else {
                                tokens.append(quote! {
                                    .#param_ident(&[#(#result),*])
                                });
                            }
                        }
                        _ => println!("Unsupported value {:?}", v),
                    }
                }

                Ok(Some(tokens))
            }
        }
    }

    fn generate_parts(
        api_call: &str,
        endpoint: &ApiEndpoint,
        parts: &[(&str, &Yaml)],
    ) -> Result<Option<Tokens>, failure::Error> {
        // TODO: ideally, this should share the logic from EnumBuilder
        let enum_name = {
            let name = api_call.to_pascal_case().replace(".", "");
            syn::Ident::from(format!("{}Parts", name))
        };

        // Enum variants containing no URL parts where there is only a single API URL,
        // are not required to be passed in the API
        if parts.is_empty() {
            let param_counts = endpoint
                .url
                .paths
                .iter()
                .map(|p| p.path.params().len())
                .collect::<Vec<usize>>();

            if !param_counts.contains(&0) {
                return Err(failure::err_msg(format!(
                    "No path for '{}' API with no URL parts",
                    api_call
                )));
            }

            return match endpoint.url.paths.len() {
                1 => Ok(None),
                _ => Ok(Some(quote!(#enum_name::None))),
            };
        }

        let path = match endpoint.url.paths.len() {
            1 => {
                let path = &endpoint.url.paths[0];
                if path.path.params().len() == parts.len() {
                    Some(path)
                } else {
                    None
                }
            }
            _ => {
                // get the matching path parts
                let matching_path_parts = endpoint
                    .url
                    .paths
                    .iter()
                    .filter(|path| {
                        let p = path.path.params();
                        if p.len() != parts.len() {
                            return false;
                        }

                        let contains = parts
                            .iter()
                            .filter_map(|i| if p.contains(&i.0) { Some(()) } else { None })
                            .collect::<Vec<_>>();
                        contains.len() == parts.len()
                    })
                    .collect::<Vec<_>>();

                match matching_path_parts.len() {
                    0 => None,
                    _ => Some(matching_path_parts[0]),
                }
            }
        };

        if path.is_none() {
            return Err(failure::err_msg(format!(
                "No path for '{}' API with URL parts {:?}",
                &api_call, parts
            )));
        }

        let path = path.unwrap();
        let path_parts = path.path.params();
        let variant_name = {
            let v = path_parts
                .iter()
                .map(|k| k.to_pascal_case())
                .collect::<Vec<_>>()
                .join("");
            syn::Ident::from(v)
        };

        let part_tokens: Vec<Result<Tokens, failure::Error>> = parts
            .iter()
            // don't rely on URL parts being ordered in the yaml test
            .sorted_by(|(p, _), (p2, _)| {
                let f = path_parts.iter().position(|x| x == p).unwrap();
                let s = path_parts.iter().position(|x| x == p2).unwrap();
                f.cmp(&s)
            })
            .map(|(p, v)| {
                let ty = match path.parts.get(*p) {
                    Some(t) => Ok(t),
                    None => Err(failure::err_msg(format!(
                        "No URL part found for {} in {}",
                        p, &path.path
                    ))),
                }?;

                match v {
                    Yaml::String(s) => match ty.ty {
                        TypeKind::List => {
                            let values: Vec<&str> = s.split(',').collect();
                            Ok(quote! { &[#(#values),*] })
                        }
                        TypeKind::Long => {
                            let l = s.parse::<i64>().unwrap();
                            Ok(quote! { #l })
                        }
                        _ => Ok(quote! { #s }),
                    },
                    Yaml::Boolean(b) => {
                        let s = b.to_string();
                        Ok(quote! { #s })
                    }
                    Yaml::Integer(i) => match ty.ty {
                        TypeKind::Long => Ok(quote! { #i }),
                        _ => {
                            let s = i.to_string();
                            Ok(quote! { #s })
                        }
                    },
                    Yaml::Array(arr) => {
                        // only support param string arrays
                        let result: Vec<_> = arr
                            .iter()
                            .map(|i| match i {
                                Yaml::String(s) => Ok(s),
                                y => Err(failure::err_msg(format!(
                                    "Unsupported array value {:?}",
                                    y
                                ))),
                            })
                            .collect();

                        match ok_or_accumulate(&result, 0) {
                            Ok(_) => {
                                let result: Vec<_> =
                                    result.into_iter().filter_map(Result::ok).collect();

                                match ty.ty {
                                    // Some APIs specify a part is a string in the REST API spec
                                    // but is really a list, which is what a YAML test might pass
                                    // e.g. security.get_role_mapping.
                                    // see https://github.com/elastic/elasticsearch/pull/53207
                                    TypeKind::String => {
                                        let s = result.iter().join(",");
                                        Ok(quote! { #s })
                                    }
                                    _ => Ok(quote! { &[#(#result),*] }),
                                }
                            }
                            Err(e) => Err(failure::err_msg(e)),
                        }
                    }
                    _ => Err(failure::err_msg(format!("Unsupported value {:?}", v))),
                }
            })
            .collect();

        match ok_or_accumulate(&part_tokens, 0) {
            Ok(_) => {
                let part_tokens: Vec<Tokens> =
                    part_tokens.into_iter().filter_map(Result::ok).collect();
                Ok(Some(
                    quote! { #enum_name::#variant_name(#(#part_tokens),*) },
                ))
            }
            Err(e) => Err(failure::err_msg(e)),
        }
    }

    /// Creates the body function call from a YAML value.
    ///
    /// When reading a body from the YAML test, it'll be converted to a Yaml variant,
    /// usually a Hash. To get the JSON representation back requires converting
    /// back to JSON
    fn generate_body(endpoint: &ApiEndpoint, v: &Yaml) -> Option<Tokens> {
        let accepts_nd_body = match &endpoint.body {
            Some(b) => match &b.serialize {
                Some(s) => s == "bulk",
                _ => false,
            },
            None => false,
        };

        match v {
            Yaml::String(s) => {
                if accepts_nd_body {
                    Some(quote!(.body(vec![#s])))
                } else {
                    Some(quote!(.body(#s)))
                }
            }
            _ => {
                let mut s = String::new();
                {
                    let mut emitter = YamlEmitter::new(&mut s);
                    emitter.dump(v).unwrap();
                }

                if accepts_nd_body {
                    let values: Vec<serde_json::Value> = serde_yaml::from_str(&s).unwrap();
                    let json: Vec<Tokens> = values
                        .iter()
                        .map(|value| {
                            let json = serde_json::to_string(&value).unwrap();
                            let ident = syn::Ident::from(json);
                            if value.is_string() {
                                quote!(#ident)
                            } else {
                                quote!(JsonBody::from(json!(#ident)))
                            }
                        })
                        .collect();
                    Some(quote!(.body(vec![ #(#json),* ])))
                } else {
                    let value: serde_json::Value = serde_yaml::from_str(&s).unwrap();
                    let json = serde_json::to_string(&value).unwrap();

                    //let ident = syn::Ident::from(json);

                    Some(quote!(.body(#json)))
                }
            }
        }
    }
}

pub fn generate_tests_from_yaml(
    api: &Api,
    base_download_dir: &PathBuf,
    download_dir: &PathBuf,
    generated_dir: &PathBuf,
) -> Result<(), failure::Error> {
    let paths = fs::read_dir(download_dir)?;
    for entry in paths {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    generate_tests_from_yaml(api, base_download_dir, &entry.path(), generated_dir)?;
                } else if file_type.is_file() {
                    let file_name = entry.file_name().to_string_lossy().into_owned();

                    // skip non-yaml files
                    if !file_name.ends_with(".yml") && !file_name.ends_with(".yaml") {
                        continue;
                    }

                    let yaml = fs::read_to_string(&entry.path()).unwrap();
                    // a yaml test can contain multiple yaml docs
                    let result = YamlLoader::load_from_str(&yaml);
                    if result.is_err() {
                        println!(
                            "Error reading {:?}. skipping:\n\t{}",
                            &entry.path(),
                            result.err().unwrap().to_string()
                        );
                        continue;
                    }

                    let docs = result.unwrap();
                    let mut test = YamlTests::new(api.version(), docs.len());

                    let results : Vec<Result<(), failure::Error>> = docs
                        .iter()
                        .map(|doc| {
                            if let Some(hash) = doc.as_hash() {
                                let (first_key, first_value) = hash.iter().next().unwrap();
                                match (first_key, first_value) {
                                    (Yaml::String(name), Yaml::Array(steps)) => {
                                        let steps = parse_steps(api, steps)?;
                                        match name.as_str() {
                                            "setup" => test.add_setup(steps),
                                            "teardown" => test.add_teardown(steps),
                                            name => {
                                                let test_fn = YamlTestFn::new(name, steps);
                                                test.add_test_fn(test_fn)
                                            },
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
                            } else {
                                Err(failure::err_msg(format!(
                                    "expected hash but found {:?}",
                                    &doc
                                )))
                            }


                        })
                        .collect();

                    //if there has been an Err in any step of the yaml test file, don't create a test for it
                    match ok_or_accumulate(&results, 1) {
                        Ok(_) => {
                            write_test_file(test, &entry.path(), base_download_dir, generated_dir)?
                        }
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
    let paths = fs::read_dir(generated_dir).unwrap();
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

fn parse_steps(api: &Api, steps: &[Yaml]) -> Result<Vec<Step>, failure::Error> {
    let mut parsed_steps : Vec<Step> = Vec::new();
    for step in steps {
        if let Some(hash) = step.as_hash() {
            let (k, v) = hash.iter().next().unwrap();

            let key = k.as_str().unwrap();

            // TODO: pass v directly to try_parse step and handle conversion to yaml type inside
            match (key, v) {
                ("skip", Yaml::Hash(h)) => {
                    let skip = Skip::try_parse(h)?;
                    parsed_steps.push(skip.into());
                }
                ("do", Yaml::Hash(h)) => {
                    let d = Do::try_parse(api, h)?;
                    parsed_steps.push(d.into())
                },
                ("set", Yaml::Hash(_h)) => {}
                ("transform_and_set", Yaml::Hash(_h)) => {}
                ("match", Yaml::Hash(h)) => {
                    let m = Match::try_parse(h)?;
                    parsed_steps.push(m.into());
                },
                ("contains", Yaml::Hash(_h)) => {}
                ("is_true", Yaml::Hash(_h)) => {}
                ("is_true", Yaml::String(_s)) => {}
                ("is_false", Yaml::Hash(_h)) => {}
                ("is_false", Yaml::String(_s)) => {}
                ("length", Yaml::Hash(_h)) => {}
                ("eq", Yaml::Hash(_h)) => {}
                ("gte", Yaml::Hash(_h)) => {}
                ("lte", Yaml::Hash(_h)) => {}
                ("gt", Yaml::Hash(_h)) => {}
                ("lt", Yaml::Hash(_h)) => {}
                (op, _) => return Err(failure::err_msg(format!("unknown step operation: {}", op))),
            }
        } else {
            return Err(failure::err_msg(format!("{:?} is not a hash", &step)));
        }
    }

    Ok(parsed_steps)
}

pub struct Skip {
    version_requirements: Option<semver::VersionReq>,
    version: Option<String>,
    reason: Option<String>,
    features: Option<Vec<String>>,
}

impl From<Skip> for Step {
    fn from(skip: Skip) -> Self {
        Step::Skip(skip)
    }
}

impl Skip {
    fn try_parse(hash: &Hash) -> Result<Skip, failure::Error> {

        fn string_value(hash: &Hash, name: &str) -> Option<String> {
            hash
                .get(&Yaml::from_str(name))
                .map_or_else(|| None, |y| {
                    y.as_str().map(|s| s.to_string())
                })
        }

        fn array_value(hash: &Hash, name: &str) -> Option<Vec<String>> {
            hash
                .get(&Yaml::from_str(name))
                .map_or_else(|| None, |y| {
                    match y.as_str() {
                        Some(s) => Some(vec![s.to_string()]),
                        None => y.as_vec().map(|arr|
                            arr
                            .iter()
                            .map(|a| a.as_str().map(|s| s.to_string()).unwrap())
                            .collect()
                        )
                    }

                })
        }

        let version = string_value(hash, "version");
        let reason = string_value(hash, "reason");
        let features = array_value(hash, "features");

        let version_requirements = if let Some(v) = &version {
            if v.to_lowercase() == "all" {
                Some(semver::VersionReq::any())
            } else {
                lazy_static! {
                    static ref VERSION_REGEX: Regex =
                        Regex::new(r"^([\w\.]+)?\s*?\-\s*?([\w\.]+)?$").unwrap();
                }
                if let Some(c) = VERSION_REGEX.captures(v) {
                    match (c.get(1), c.get(2)) {
                        (Some(start), Some(end)) => Some(
                            semver::VersionReq::parse(
                                format!(">={},<={}", start.as_str(), end.as_str()).as_ref(),
                            ).unwrap(),
                        ),
                        (Some(start), None) => Some(
                            semver::VersionReq::parse(format!(">={}", start.as_str()).as_ref()).unwrap(),
                        ),
                        (None, Some(end)) => Some(
                            semver::VersionReq::parse(format!("<={}", end.as_str()).as_ref()).unwrap()
                        ),
                        (None, None) => None,
                    }
                } else {
                    None
                }
            }
        } else {
            None
        };

        Ok(Skip {
            version,
            version_requirements,
            reason,
            features,
        })
    }

    pub fn matches(&self, version: &semver::Version) -> bool {
        match &self.version_requirements {
            Some(r) => r.matches(version),
            None => false,
        }
    }
}

pub struct Match {
    hash: Hash
}

impl From<Match> for Step {
    fn from(m: Match) -> Self {
        Step::Match(m)
    }
}

impl Match {
    pub fn try_parse(hash: &Hash) -> Result<Match, failure::Error> {
        Ok(Match { hash: hash.clone() })
    }

    /// Builds an indexer expression from the match key
    fn get_expr(key: &str) -> String {
        if key == "$body" {
            key.into()
        } else {
            let mut values = Vec::new();
            let mut value = String::new();
            let mut chars = key.chars();
            while let Some(ch) = chars.next() {
                match ch {
                    '\\' => {
                        // consume the next character too
                        if let Some(next) = chars.next() {
                            value.push(next);
                        }
                    }
                    '.' => {
                        values.push(value);
                        value = String::new();
                    }
                    _ => {
                        value.push(ch);
                    }
                }
            }
            values.push(value);
            let mut expr = String::new();
            for s in values {
                if s.chars().all(char::is_numeric) {
                    write!(expr, "[{}]", s).unwrap();
                } else {
                    write!(expr, "[\"{}\"]", s).unwrap();
                }
            };
            expr
        }
    }
}

impl ToTokens for Match {
    // TODO: Move this parsing out into Match::try_parse
    fn to_tokens(&self, tokens: &mut Tokens) {
        let (k, v) = self.hash.iter().next().unwrap();
        let key = k.as_str().unwrap().trim();
        let expr = Self::get_expr(key);

        match v {
            Yaml::String(s) => {
                if s.starts_with('/') {
                    let s = s.trim().trim_matches('/');
                    if expr == "$body" {
                        tokens.append(quote! {
                            let string_response_body = serde_json::to_string(&response_body).unwrap();
                            let regex = regex::Regex::new(#s)?;
                            assert!(
                                regex.is_match(&string_response_body),
                                "expected $body:\n\n{}\n\nto match regex:\n\n{}",
                                &string_response_body,
                                #s
                            );
                        });
                    } else {
                        let ident = syn::Ident::from(expr.clone());
                        tokens.append(quote! {
                            let regex = regex::Regex::new(#s)?;
                            assert!(
                                regex.is_match(response_body#ident.as_str().unwrap()),
                                "expected value at {}:\n\n{}\n\nto match regex:\n\n{}",
                                #expr,
                                response_body#ident.as_str().unwrap(),
                                #s
                            );
                        });
                    }
                } else {
                    let ident = syn::Ident::from(expr.clone());
                    tokens.append(quote! {
                        assert_eq!(
                            response_body#ident.as_str().unwrap(),
                            #s,
                            "expected value at {} to be {} but was {}",
                            #expr,
                            #s,
                            response_body#ident.as_str().unwrap()
                        );
                    })
                }
            },
            Yaml::Integer(i) => {
                if expr == "$body" {
                    panic!("match on $body with integer");
                } else {
                    let ident = syn::Ident::from(expr.clone());
                    tokens.append(quote! {
                        assert_eq!(
                            response_body#ident.as_i64().unwrap(),
                            #i,
                            "expected value at {} to be {} but was {}",
                            #expr,
                            #i,
                            response_body#ident.as_i64().unwrap()
                        );
                    });
                }
            }
            // TODO: handle hashes, etc.
            _ => {}
        }
    }
}

pub enum Step {
    Skip(Skip),
    Do(Do),
    Match(Match),
}

impl Step {
    pub fn r#do(&self) -> Option<&Do> {
        match self {
            Step::Do(d) => Some(d),
            _ => None
        }
    }
}

pub struct Do {
    headers: BTreeMap<String, String>,
    catch: Option<String>,
    api_call: ApiCall,
    warnings: Vec<String>,
}

impl ToTokens for Do {
    fn to_tokens(&self, tokens: &mut Tokens) {

        // TODO: Add in catch, headers, warnings
        &self.api_call.to_tokens(tokens);
    }
}

impl From<Do> for Step {
    fn from(d: Do) -> Self {
        Step::Do(d)
    }
}

impl Do {
    pub fn try_parse(
        api: &Api,
        hash: &Hash,
    ) -> Result<Do, failure::Error> {
        let mut api_call: Option<ApiCall> = None;
        let mut headers = BTreeMap::new();
        let mut warnings: Vec<String> = Vec::new();
        let mut catch = None;

        let results: Vec<Result<(), failure::Error>> = hash
            .iter()
            .map(|(k, v)| {
                match k.as_str() {
                    Some(key) => {
                        match key {
                            "headers" => {
                                match v.as_hash() {
                                    Some(h) => {
                                        //for (k, v) in h.iter() {}


                                        Ok(())
                                    },
                                    None => Err(failure::err_msg(format!(
                                        "expected hash but found {:?}",
                                        v
                                    ))),
                                }
                            }
                            "catch" => {
                                catch = v.as_str().map(|s| s.to_string());
                                Ok(())
                            }
                            "node_selector" => {
                                // TODO: implement
                                Ok(())
                            }
                            "warnings" => {
                                warnings = v
                                    .as_vec()
                                    .map(|a| a.iter().map(|y| y.as_str().unwrap().to_string()).collect())
                                    .unwrap();
                                Ok(())
                            }
                            call => {
                                let hash = v.as_hash();
                                if hash.is_none() {
                                    return Err(failure::err_msg(format!(
                                        "expected hash value for {} but found {:?}",
                                        &call, v
                                    )));
                                }

                                let endpoint = match api.endpoint_for_api_call(call) {
                                    Some(e) => Ok(e),
                                    None => {
                                        Err(failure::err_msg(format!("no API found for {}", call)))
                                    }
                                }?;

                                api_call = Some(ApiCall::try_from(api, endpoint, hash.unwrap())?);
                                Ok(())
                            }
                        }
                    }
                    None => Err(failure::err_msg(format!(
                        "expected string key but found {:?}",
                        k
                    ))),
                }
            })
            .collect();

        ok_or_accumulate(&results, 0)?;

        Ok(Do {
            api_call: api_call.unwrap(),
            catch,
            headers,
            warnings,
        })
    }
}

/// Checks whether there are any Errs in the collection, and accumulates them into one
/// error message if there are.
fn ok_or_accumulate<T>(
    results: &[Result<T, failure::Error>],
    indent: usize,
) -> Result<(), failure::Error> {
    let errs = results
        .iter()
        .filter_map(|r| r.as_ref().err())
        .collect::<Vec<_>>();
    if errs.is_empty() {
        Ok(())
    } else {
        let msg = errs
            .iter()
            .map(|e| format!("{}{}", "\t".to_string().repeat(indent), e.to_string()))
            .collect::<Vec<_>>()
            .join("\n");

        Err(failure::err_msg(msg))
    }
}
