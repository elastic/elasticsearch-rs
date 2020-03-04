use inflector::Inflector;
use quote::Tokens;

use api_generator::generator::{Api, ApiEndpoint};
use itertools::Itertools;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use syn::parse::path;
use yaml_rust::{
    yaml::{Array, Hash},
    Yaml, YamlEmitter, YamlLoader,
};

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

/// The components of an API call
struct ApiCall<'a> {
    parts: Vec<(&'a str, &'a Yaml)>,
    params: Vec<(&'a str, &'a Yaml)>,
    body_call: Option<Tokens>,
    ignore: Option<i64>
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
                            "error reading {:?}: {}. skipping",
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
                                        let tokens = read_steps(api, steps)?;
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
                    match ok_or_accumulate(results, 1) {
                        Ok(_) => {
                            write_test_file(test, &entry.path(), base_download_dir, generated_dir)?
                        }
                        Err(e) => {
                            println!("error(s) creating test file for {:?}\n{}", &entry.path(), e)
                        }
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
        // directories and files will form the module names so ensure they're valid
        let clean: String = relative
            .to_string_lossy()
            .into_owned()
            .replace(".", "_")
            .replace("-", "_");
        relative = PathBuf::from(clean);

        let mut path = generated_dir.join(relative);
        path.set_extension("rs");
        // modules can't start with a number
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

    let (setup_fn, setup_call) = if let Some(s) = &test.setup {
        (
            Some(quote! {
                async fn setup(client: &Elasticsearch) -> Result<(), failure::Error> {
                    #s
                }
            }),
            Some(quote! { setup(&client).await?; }),
        )
    } else {
        (None, None)
    };

    let (teardown_fn, teardown_call) = if let Some(t) = &test.teardown {
        (
            Some(quote! {
                async fn teardown(client: &Elasticsearch) -> Result<(), failure::Error> {
                    #t
                }
            }),
            Some(quote! { teardown(&client).await?; }),
        )
    } else {
        (None, None)
    };

    let tests: Vec<Tokens> = test
        .tests
        .iter()
        .map(|(name, steps)| {
            let method_name = name.replace(" ", "_").to_lowercase().to_snake_case();

            let method_name_ident = syn::Ident::from(method_name);
            quote! {
                #[tokio::test]
                async fn #method_name_ident() -> Result<(), failure::Error> {
                    let client = client::create();
                    #setup_call
                    #steps
                    #teardown_call
                    Ok(())
                }
            }
        })
        .collect();

    let tokens = quote! {
        #[cfg(test)]
        pub mod tests {
            use elasticsearch::*;
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

fn read_steps(api: &Api, steps: &Array) -> Result<Tokens, failure::Error> {
    let mut tokens = Tokens::new();
    for step in steps {
        if let Some(hash) = step.as_hash() {
            let (k, v) = hash.iter().next().unwrap();

            let key = k.as_str().unwrap();

            match (key, v) {
                ("skip", Yaml::Hash(h)) => {}
                ("do", Yaml::Hash(h)) => read_do(api, h, &mut tokens)?,
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

fn read_do(api: &Api, hash: &Hash, tokens: &mut Tokens) -> Result<(), failure::Error> {
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
                            let components = api_call_components(api, endpoint, hash.unwrap());

                            let parts_variant = parts_variant(api_call, &components.parts, endpoint)?;

                            let params_calls = match &components.params.len() {
                                0 => None,
                                _ => {
                                    let mut tokens = Tokens::new();
                                    for (n, v) in &components.params {
                                        let param_ident = syn::Ident::from(
                                            api_generator::generator::code_gen::valid_name(n),
                                        );

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

    ok_or_accumulate(results, 0)
}

fn api_call_components<'a>(api: &'a Api, endpoint: &'a ApiEndpoint, hash: &'a Hash) -> ApiCall<'a> {
    let mut parts: Vec<(&str, &Yaml)> = vec![];
    let mut params: Vec<(&str, &Yaml)> = vec![];
    let mut body_call: Option<Tokens> = None;
    // TODO: use ignore value in the test
    let mut ignore: Option<i64> = None;

    for (k, v) in hash.iter() {
        let key = k.as_str().unwrap();
        if endpoint.params.contains_key(key)
            || api.common_params.contains_key(key)
        {
            params.push((key, v));
        } else if key == "body" {
            body_call = create_body_call(v);
        } else if key == "ignore" {
            ignore = Some(v.as_i64().unwrap());
        } else {
            parts.push((key, v));
        }
    }

    ApiCall {
        parts,
        params,
        body_call,
        ignore
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

    // Enum variants containing no URL parts where there is only a single API URL are not
    // required to be passed in the API
    if parts.is_empty() {
        return match endpoint.url.paths.len() {
            1 => Ok(None),
            _ => Ok(Some(quote!(#enum_name::None))),
        };
    }

    let path_parts = match endpoint.url.paths.len() {
        1 => Some(endpoint.url.paths[0].path.params()),
        _ => {
            let paths: Vec<Vec<_>> = endpoint.url.paths.iter().map(|p| p.path.params()).collect();

            // get the matching path parts
            let matching_path_parts = paths
                .into_iter()
                .filter(|p| {
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
                _ => Some(matching_path_parts[0].clone()),
            }
        }
    };

    if path_parts.is_none() {
        return Err(failure::err_msg(format!(
            "No path_parts for {} with parts {:?}",
            &api_call, parts
        )));
    }

    let path_parts = path_parts.unwrap();
    let variant_name = {
        let v = path_parts
            .iter()
            .map(|k| k.to_pascal_case())
            .collect::<Vec<_>>()
            .join("");
        syn::Ident::from(v)
    };

    let part_tokens = parts
        .clone()
        .into_iter()
        // don't rely on URL parts being ordered in the yaml test
        .sorted_by(|(p, v), (p2, v2)| {
            let f = path_parts.iter().position(|x| x == p).unwrap();
            let s = path_parts.iter().position(|x| x == p2).unwrap();
            f.cmp(&s)
        })
        .map(|(p, v)| {
            match v {
                Yaml::String(s) => quote! { #s },
                Yaml::Boolean(b) => quote! { #b },
                Yaml::Integer(i) => quote! { #i },
                Yaml::Array(arr) => {
                    // only support param string arrays
                    let result: Vec<_> = arr
                        .iter()
                        .map(|i| match i {
                            Yaml::String(s) => Ok(s),
                            y => Err(failure::err_msg(format!("Unsupported array value {:?}", y))),
                        })
                        .filter_map(Result::ok)
                        .collect();
                    quote! { &[#(#result,)*] }
                }
                _ => panic!(format!("Unsupported value {:?}", v)),
            }
        })
        .collect::<Vec<Tokens>>();

    Ok(Some(
        quote! { #enum_name::#variant_name(#(#part_tokens,)*) },
    ))
}

/// Creates the body function call from a YAML value.
///
/// When reading a body from the YAML test, it'll be converted to a Yaml variant,
/// usually a Hash. To get the JSON representation back requires converting
/// back to JSON
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
                "No ApiEndpoint found for {}. skipping",
                &api_call
            ))),
        },
        _ => match api.namespaces.get(api_call_path[0]) {
            Some(namespace) => match namespace.get(api_call_path[1]) {
                Some(endpoint) => Ok(endpoint),
                None => Err(failure::err_msg(format!(
                    "No ApiEndpoint found for {}. skipping",
                    &api_call
                ))),
            },
            None => Err(failure::err_msg(format!(
                "No ApiEndpoint found for {}. skipping",
                &api_call
            ))),
        },
    }
}

fn ok_or_accumulate(results: Vec<Result<(), failure::Error>>, indent: usize) -> Result<(), failure::Error> {
    let errs = results
        .into_iter()
        .filter_map(|r| r.err())
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
