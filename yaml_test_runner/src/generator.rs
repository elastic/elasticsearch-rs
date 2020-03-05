use inflector::Inflector;
use quote::Tokens;

use api_generator::generator::{Api, ApiEndpoint, TypeKind, ApiEnum};
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use syn::parse::path;
use yaml_rust::{
    yaml::{Array, Hash},
    Yaml, YamlEmitter, YamlLoader,
};
use std::collections::HashSet;

struct YamlTest {
    namespaces: HashSet<String>,
    setup: Option<Tokens>,
    teardown: Option<Tokens>,
    tests: Vec<(String, Tokens)>,
}

impl YamlTest {
    pub fn new(len: usize) -> Self {
        Self {
            namespaces: HashSet::with_capacity(len),
            setup: None,
            teardown: None,
            tests: Vec::with_capacity(len),
        }
    }
}

/// The components of an API call
struct ApiCall<'a> {
    namespace: Option<&'a str>,
    parts: Vec<(&'a str, &'a Yaml)>,
    params: Vec<(&'a str, &'a Yaml)>,
    body_call: Option<Tokens>,
    ignore: Option<i64>,
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
                    let mut test = YamlTest::new(docs.len());

                    let results : Vec<Result<(), failure::Error>> = docs
                        .iter()
                        .map(|doc| {
                            if let Some(hash) = doc.as_hash() {
                                let (first_key, first_value) = hash.iter().next().unwrap();
                                match (first_key, first_value) {
                                    (Yaml::String(name), Yaml::Array(steps)) => {
                                        let tokens = read_steps(api, &mut test, steps)?;
                                        match name.as_str() {
                                            "setup" => test.setup = Some(tokens),
                                            "teardown" => test.teardown = Some(tokens),
                                            name => test.tests.push((name.to_owned(), tokens)),
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
    test: YamlTest,
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

    let (setup_fn, setup_call) =
        generate_fixture("setup", &test.setup);
    let (teardown_fn, teardown_call) =
        generate_fixture("teardown", &test.teardown);

    let mut method_names = HashSet::new();

    let tests: Vec<Tokens> = test
        .tests
        .iter()
        .map(|(name, steps)| {
            let method_name = {
                let mut method_name = name.replace(" ", "_").to_lowercase().to_snake_case();

                // some method descriptions are the same in YAML tests, which would result in
                // duplicate generated test function names. Deduplicate by appending incrementing number
                while !method_names.insert(method_name.clone()) {
                    lazy_static! {
                        static ref ENDING_DIGITS_REGEX: Regex = Regex::new(r"^(.*?)_(\d*?)$").unwrap();
                    }
                    if let Some(c) = ENDING_DIGITS_REGEX.captures(&method_name) {
                        let name = c.get(1).unwrap().as_str();
                        let n = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
                        method_name = format!("{}_{}", name, n + 1);
                    } else {
                        method_name = format!("{}_2", method_name);
                    }
                }
                syn::Ident::from(method_name)
            };
            quote! {
                #[tokio::test]
                async fn #method_name() -> Result<(), failure::Error> {
                    let client = client::create();
                    #setup_call
                    #steps
                    #teardown_call
                    Ok(())
                }
            }
        })
        .collect();

    let namespaces: Vec<Tokens> = test.namespaces
        .iter()
        .map(|n| {
            let ident = syn::Ident::from(n.as_str());
            quote!(use elasticsearch::#ident::*;)
        })
        .collect();

    let tokens = quote! {
        #[cfg(test)]
        pub mod tests {
            use elasticsearch::*;
            use elasticsearch::params::*;
            #(#namespaces)*
            use crate::client;

            #setup_fn
            #teardown_fn
            #(#tests)*
        }
    };

    let generated = api_generator::generator::rust_fmt(tokens.to_string())?;
    let mut file = OpenOptions::new().append(true).open(&path)?;
    file.write_all(generated.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}

fn generate_fixture(name: &str, tokens: &Option<Tokens>) -> (Option<Tokens>, Option<Tokens>) {
    if let Some(t) = tokens {
        let ident = syn::Ident::from(name);
        (
            Some(quote! {
                async fn #ident(client: &Elasticsearch) -> Result<(), failure::Error> {
                    #t
                    Ok(())
                }
            }),
            Some(quote! { #ident(&client).await?; }),
        )
    } else {
        (None, None)
    }
}

fn read_steps(api: &Api, test: &mut YamlTest, steps: &Array) -> Result<Tokens, failure::Error> {
    let mut tokens = Tokens::new();
    for step in steps {
        if let Some(hash) = step.as_hash() {
            let (k, v) = hash.iter().next().unwrap();

            let key = k.as_str().unwrap();

            match (key, v) {
                ("skip", Yaml::Hash(h)) => {}
                ("do", Yaml::Hash(h)) => read_do(api, test, h, &mut tokens)?,
                ("set", Yaml::Hash(h)) => {}
                ("transform_and_set", Yaml::Hash(h)) => {}
                ("match", Yaml::Hash(h)) => read_match(api, h, &mut tokens)?,
                ("contains", Yaml::Hash(h)) => {}
                ("is_true", Yaml::Hash(h)) => {}
                ("is_true", Yaml::String(s)) => {}
                ("is_false", Yaml::Hash(h)) => {}
                ("is_false", Yaml::String(s)) => {}
                ("length", Yaml::Hash(h)) => {}
                ("eq", Yaml::Hash(h)) => {}
                ("gte", Yaml::Hash(h)) => {}
                ("lte", Yaml::Hash(h)) => {}
                ("gt", Yaml::Hash(h)) => {}
                ("lt", Yaml::Hash(h)) => {}
                (op, _) => return Err(failure::err_msg(format!("unknown step operation: {}", op))),
            }
        } else {
            return Err(failure::err_msg(format!("{:?} is not a hash", &step)));
        }
    }

    Ok(tokens)
}

fn read_match(api: &Api, hash: &Hash, tokens: &mut Tokens) -> Result<(), failure::Error> {
    // TODO: implement
    Ok(())
}

fn read_do(api: &Api, test: &mut YamlTest, hash: &Hash, tokens: &mut Tokens) -> Result<(), failure::Error> {
    let results: Vec<Result<(), failure::Error>> = hash
        .iter()
        .map(|(k, v)| {
            match k.as_str() {
                Some(key) => {
                    match key {
                        "headers" => {
                            // TODO: implement
                            Ok(())
                        }
                        "catch" => {
                            // TODO: implement
                            Ok(())
                        }
                        "node_selector" => {
                            // TODO: implement
                            Ok(())
                        }
                        "warnings" => {
                            // TODO: implement
                            Ok(())
                        }
                        api_call => {
                            let hash = v.as_hash();
                            if hash.is_none() {
                                return Err(failure::err_msg(format!(
                                    "expected hash value for {} but found {:?}",
                                    &api_call, v
                                )));
                            }

                            let endpoint = endpoint_from_api_call(api, &api_call)?;
                            let components = api_call_components(api, endpoint, api_call, hash.unwrap());

                            if let Some(n) = components.namespace {
                                test.namespaces.insert(n.to_owned());
                            }

                            // TODO: move into components construction
                            let parts_variant =
                                parts_variant(api_call, &components.parts, endpoint)?;

                            // TODO: move into components construction
                            let params_calls = match &components.params.len() {
                                0 => None,
                                _ => {
                                    let mut tokens = Tokens::new();
                                    for (n, v) in &components.params {
                                        let param_ident = syn::Ident::from(
                                            api_generator::generator::code_gen::valid_name(n),
                                        );

                                        let ty = match endpoint.params.get(*n) {
                                            Some(t) => Ok(t),
                                            None => match api.common_params.get(*n) {
                                                Some(t) => Ok(t),
                                                None => Err(failure::err_msg(format!("No param found for {}", n)))
                                            }
                                        }?;

                                        let kind = ty.ty;

                                        match v {
                                            Yaml::String(s) => {
                                                match kind {
                                                    TypeKind::Enum => {
                                                        let e: String = n.to_pascal_case();
                                                        let enum_name = syn::Ident::from(e.as_str());
                                                        let variant = if s.is_empty() {
                                                            // TODO: Should we simply omit empty Refresh tests?
                                                            if e == "Refresh" {
                                                                syn::Ident::from("True")
                                                            } else if e == "Size" {
                                                                syn::Ident::from("Unspecified")
                                                            } else {
                                                                panic!(format!("Unhandled empty value for {}", &e));
                                                            }
                                                        } else {
                                                            syn::Ident::from(s.to_pascal_case().replace("_", ""))
                                                        };

                                                        tokens.append(quote! {
                                                            .#param_ident(#enum_name::#variant)
                                                        })
                                                    },
                                                    TypeKind::List => {
                                                        let values: Vec<&str> = s.split(',').collect();
                                                        tokens.append(quote! {
                                                        .#param_ident(&[#(#values),*])
                                                        })
                                                    },
                                                    _ => tokens.append(quote! {
                                                        .#param_ident(#s)
                                                    }),
                                                }
                                            },
                                            Yaml::Boolean(b) => {
                                                match kind {
                                                    TypeKind::Enum => {
                                                        let enum_name = syn::Ident::from(n.to_pascal_case());
                                                        let variant = syn::Ident::from(b.to_string().to_pascal_case());
                                                        tokens.append(quote! {
                                                            .#param_ident(#enum_name::#variant)
                                                        })
                                                    }
                                                    _ => tokens.append(quote! {
                                                        .#param_ident(#b)
                                                    }),
                                                }
                                            },
                                            Yaml::Integer(i) => tokens.append(quote! {
                                                .#param_ident(#i)
                                            }),
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
                                                    .filter_map(Result::ok)
                                                    .collect();

                                                tokens.append(quote! {
                                                    .#param_ident(&[#(#result,)*])
                                                });
                                            }
                                            _ => println!("Unsupported value {:?}", v),
                                        }
                                    }

                                    Some(tokens)
                                }
                            };

                            let fn_call = syn::Ident::from(api_call.replace(".", "()."));
                            let body_call = components.body_call;

                            tokens.append(quote! {
                                let response = client.#fn_call(#parts_variant)
                                    #params_calls
                                    #body_call
                                    .send()
                                    .await?;
                            });
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

    ok_or_accumulate(&results, 0)
}

fn api_call_components<'a>(api: &'a Api, endpoint: &'a ApiEndpoint, api_call: &'a str, hash: &'a Hash) -> ApiCall<'a> {
    let mut parts: Vec<(&str, &Yaml)> = vec![];
    let mut params: Vec<(&str, &Yaml)> = vec![];
    let mut body_call: Option<Tokens> = None;
    // TODO: use ignore value in the test
    let mut ignore: Option<i64> = None;

    for (k, v) in hash.iter() {
        let key = k.as_str().unwrap();
        if endpoint.params.contains_key(key) || api.common_params.contains_key(key) {
            params.push((key, v));
        } else if key == "body" {
            body_call = create_body_call(endpoint, v);
        } else if key == "ignore" {
            ignore = match v.as_i64() {
                Some(i) => Some(i),
                None => v.as_vec().unwrap()[0].as_i64(),
            }
        } else {
            parts.push((key, v));
        }
    }

    let namespace: Option<&str> = if api_call.contains(".") {
        Some(api_call.splitn(2, ".").collect::<Vec<_>>()[0])
    } else {
        None
    };

    ApiCall {
        namespace,
        parts,
        params,
        body_call,
        ignore,
    }
}

fn parts_variant(
    api_call: &str,
    parts: &Vec<(&str, &Yaml)>,
    endpoint: &ApiEndpoint,
) -> Result<Option<Tokens>, failure::Error> {
    // TODO: ideally, this should share the logic from EnumBuilder
    let enum_name = {
        let name = api_call.to_pascal_case().replace(".", "");
        syn::Ident::from(format!("{}Parts", name))
    };

    // Enum variants containing no URL parts where there is only a single API URL,
    // are not required to be passed in the API
    if parts.is_empty() {
        return match endpoint.url.paths.len() {
            1 => Ok(None),
            _ => Ok(Some(quote!(#enum_name::None))),
        };
    }

    let path = match endpoint.url.paths.len() {
        1 => Some(&endpoint.url.paths[0]),
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
        .clone()
        .into_iter()
        // don't rely on URL parts being ordered in the yaml test
        .sorted_by(|(p, _), (p2, _)| {
            let f = path_parts.iter().position(|x| x == p).unwrap();
            let s = path_parts.iter().position(|x| x == p2).unwrap();
            f.cmp(&s)
        })
        .map(|(p, v)| {
            let ty = match path.parts.get(p) {
                Some(t) => Ok(t),
                None => Err(failure::err_msg(format!("No URL part found for {} in {}", p, &path.path)))
            }?;

            match v {
                Yaml::String(s) => match ty.ty {
                    TypeKind::List => {
                        let values: Vec<&str> = s.split(',').collect();
                        Ok(quote! { &[#(#values),*] })
                    },
                    _ => Ok(quote! { #s }),
                },
                Yaml::Boolean(b) => Ok(quote! { #b }),
                Yaml::Integer(i) => Ok(quote! { #i }),
                Yaml::Array(arr) => {
                    // only support param string arrays
                    let result: Vec<_> = arr
                        .iter()
                        .map(|i| match i {
                            Yaml::String(s) => Ok(s),
                            y => Err(failure::err_msg(format!("Unsupported array value {:?}", y))),
                        })
                        .collect();

                    match ok_or_accumulate(&result, 0) {
                        Ok(_) => {
                            let result: Vec<_> =
                                result.into_iter().filter_map(Result::ok).collect();
                            Ok(quote! { &[#(#result,)*] })
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
            let part_tokens: Vec<Tokens> = part_tokens.into_iter().filter_map(Result::ok).collect();
            Ok(Some(
                quote! { #enum_name::#variant_name(#(#part_tokens,)*) },
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
fn create_body_call(endpoint: &ApiEndpoint, v: &Yaml) -> Option<Tokens> {
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
        },
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
                        quote!(json!(#ident))
                    })
                    .collect();
                Some(quote!(.body(vec![ #(#json),* ])))
            } else {
                let value: serde_json::Value = serde_yaml::from_str(&s).unwrap();
                let json = serde_json::to_string(&value).unwrap();
                let ident = syn::Ident::from(json);
                Some(quote!(.body(json!(#ident))))
            }
        }
    }
}

/// Find the right ApiEndpoint from the REST API specs for the API call
/// defined in the YAML test.
///
/// The REST API specs model only the stable APIs
/// currently, so no endpoint will be found for experimental or beta APIs
fn endpoint_from_api_call<'a>(
    api: &'a Api,
    api_call: &str,
) -> Result<&'a ApiEndpoint, failure::Error> {
    let api_call_path: Vec<&str> = api_call.split('.').collect();
    match api_call_path.len() {
        1 => match api.root.get(api_call_path[0]) {
            Some(endpoint) => Ok(endpoint),
            None => Err(failure::err_msg(format!(
                "No ApiEndpoint found for {}",
                &api_call
            ))),
        },
        _ => match api.namespaces.get(api_call_path[0]) {
            Some(namespace) => match namespace.get(api_call_path[1]) {
                Some(endpoint) => Ok(endpoint),
                None => Err(failure::err_msg(format!(
                    "No ApiEndpoint found for {}",
                    &api_call
                ))),
            },
            None => Err(failure::err_msg(format!(
                "No ApiEndpoint found for {}",
                &api_call
            ))),
        },
    }
}

/// Checks whether there are any Errs in the collection, and accumulates them into one
/// error message if there are.
fn ok_or_accumulate<T>(
    results: &[Result<T, failure::Error>],
    indent: usize,
) -> Result<(), failure::Error> {
    let errs = results
        .into_iter()
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
