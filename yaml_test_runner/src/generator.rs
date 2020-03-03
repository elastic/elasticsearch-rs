use quote::Tokens;

use std::fs;
use std::path::PathBuf;
use yaml_rust::{yaml::{Array, Hash}, Yaml, YamlLoader, YamlEmitter};
use api_generator::generator::{Api, ApiEndpoint};
use std::fs::{File, OpenOptions};
use std::io::Write;

struct YamlTest {
    setup: Option<Tokens>,
    teardown: Option<Tokens>,
    tests: Vec<(String, Tokens)>,
}

impl YamlTest {
    pub fn new(len: usize) -> Self {
        Self {
            setup: None,
            teardown: None,
            tests: Vec::with_capacity(len),
        }
    }
}

pub fn generate_tests_from_yaml(api: &Api, base_download_dir: &PathBuf, download_dir: &PathBuf, generated_dir: &PathBuf) -> Result<(), failure::Error> {
    let paths = fs::read_dir(download_dir).unwrap();
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
                        println!("error reading {:?}: {}", &entry.path(), result.err().unwrap().to_string());
                        continue;
                    }

                    let docs = result.unwrap();
                    let mut test = YamlTest::new(docs.len());

                    for doc in docs {
                        if let Some(hash) = doc.as_hash() {

                            let (first_key, first_value) = hash.iter().next().unwrap();
                            match (first_key, first_value) {
                                (Yaml::String(name), Yaml::Array(steps)) => {
                                    let tokens = read_steps(api, steps)?;
                                    match name.as_str() {
                                        "setup" => test.setup = Some(tokens),
                                        "teardown" => test.teardown = Some(tokens),
                                        name => test.tests.push((name.to_owned(), tokens)),
                                    };
                                }
                                (k, v) => {
                                    return Err(failure::err_msg(format!(
                                        "Expected string key and array value in {:?}, but found {:?} and {:?}",
                                        &entry.path(),
                                        &k,
                                        &v,
                                    )));
                                }
                            }
                        } else {
                            return Err(failure::err_msg(format!(
                                "Expected hash but found {:?} in {:?}",
                                &doc,
                                &entry.path()
                            )));
                        }
                    }

                    write_test(test, &entry.path(), base_download_dir, generated_dir)?;
                }
            }
        }
    }

    Ok(())
}

fn write_test(test: YamlTest, path: &PathBuf, base_download_dir: &PathBuf, generated_dir: &PathBuf) -> Result<(), failure::Error> {
    let path = {
        let yaml_file: String = path.to_string_lossy().into_owned();
        let file = yaml_file.replace(base_download_dir.to_str().unwrap(), generated_dir.to_str().unwrap());
        let mut path = PathBuf::from(file);
        path.set_extension("rs");
        path
    };

    fs::create_dir_all(&path.parent().unwrap())?;
    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
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

    Ok(())
}



fn read_steps(api: &Api, steps: &Array) -> Result<Tokens, failure::Error> {
    let mut tokens = Tokens::new();
    for step in steps {
        if let Some(hash) = step.as_hash() {
            let (k, v) = hash.iter().next().unwrap();

            let key = k.as_str().unwrap();

            match (key, v) {
                ("skip", Yaml::Hash(h)) => {}
                ("do", Yaml::Hash(h)) => read_do(api,h, &mut tokens)?,
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
    Ok(())
}

fn read_do(api: &Api, hash: &Hash, tokens: &mut Tokens) -> Result<(), failure::Error> {
    let results: Vec<Result<(), failure::Error>> = hash
        .iter()
        .map(|(k, v)| {
            match k.as_str() {
                Some(key) => {
                    match key {
                        "headers" => Ok(()),
                        "catch" => Ok(()),
                        "node_selector" => Ok(()),
                        "warnings" => Ok(()),
                        api_call => {

                            let c = v.as_hash();
                            if c.is_none() {
                                return Err(failure::err_msg(format!("Expected hash but found {:?}", v)));
                            }

                            let endpoint = endpoint_from_api_call(api, &api_call);

                            if endpoint.is_none() {
                                return Ok(());
                            }

                            let endpoint = endpoint.unwrap();
                            let mut parts: Vec<(&str, &Yaml)> = vec![];
                            let mut params: Vec<(&str, &Yaml)> = vec![];
                            let mut body_call: Option<Tokens> = None;

                            for (k,v) in c.unwrap().iter() {
                                let key = k.as_str().unwrap();

                                if endpoint.params.contains_key(key) {
                                    params.push((key, v));
                                } else if key == "body" {
                                    body_call = create_body_call(v);
                                } else {
                                    parts.push((key, v));
                                }
                            }

                            let fn_name = api_call.replace(".", "().");
                            let fn_name_ident = syn::Ident::from(fn_name);

                            let params_calls = match params.len() {
                                0 => None,
                                _ => {
                                    let mut tokens = Tokens::new();
                                    for (n, v) in params {
                                        let param_ident = syn::Ident::from(n);

                                        match v {
                                            Yaml::String(s) => tokens.append(quote! {
                                                .#param_ident(#s)
                                            }),
                                            Yaml::Boolean(b) => tokens.append(quote! {
                                                .#param_ident(#b)
                                            }),
                                            Yaml::Integer(i) => tokens.append(quote! {
                                                .#param_ident(#i)
                                            }),
                                            _ => println!("Unsupported value {:?}", v),
                                        }
                                    }

                                    Some(tokens)
                                }
                            };

                            tokens.append(quote! {
                                let response = client.#fn_name_ident()
                                    #params_calls
                                    #body_call
                                    .await?;
                            });
                            Ok(())
                        },
                    }
                },
                None => Err(failure::err_msg(format!("expected string key but found {:?}", k)))
            }
        })
        .collect();

    ok_or_accumulate(results)
}

fn create_body_call(v: &Yaml) -> Option<Tokens> {
    match v {
        Yaml::String(s) => Some(quote!(.body(json!(#s)))),
        _ => {
            let mut s = String::new();
            {
                let mut emitter = YamlEmitter::new(&mut s);
                emitter.dump(v).unwrap();
            }
            let v: serde_json::Value = serde_yaml::from_str(&s).unwrap();
            let json = serde_json::to_string(&v).unwrap();
            let ident = syn::Ident::from(json);
            Some(quote!(.body(json!(#ident))))
        }
    }
}

fn endpoint_from_api_call<'a>(api: &'a Api, api_call: &str) -> Option<&'a ApiEndpoint> {
    let api_call_path: Vec<&str> = api_call.split('.').collect();
    match api_call_path.len() {
        1 => match api.root.get(api_call_path[0]) {
            Some(endpoint) => Some(endpoint),
            None => {
                println!("No ApiEndpoint found for {}. skipping", api_call_path[0]);
                None
            }
        },
        _ => {
            match api.namespaces.get(api_call_path[0]) {
                Some(namespace) => {
                    match namespace.get(api_call_path[1]) {
                        Some(endpoint) => Some(endpoint),
                        None => {
                            println!("No ApiEndpoint found for {:?}. skipping", &api_call_path);
                            None
                        }
                    }
                },
                None => {
                    println!("No ApiEndpoint found for {:?}. skipping", &api_call_path);
                    None
                }
            }
        },
    }
}

fn ok_or_accumulate(
    results: Vec<Result<(), failure::Error>>,
) -> Result<(), failure::Error> {
    let errs = results
        .into_iter()
        .filter_map(|r| r.err())
        .collect::<Vec<_>>();
    if errs.is_empty() {
        Ok(())
    } else {
        let msg = errs
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        Err(failure::err_msg(msg))
    }
}
