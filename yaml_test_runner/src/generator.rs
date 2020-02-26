use quote::Tokens;

use std::fs;
use std::path::PathBuf;
use yaml_rust::{
    yaml::{Array, Hash},
    Yaml, YamlLoader,
};

struct YamlTest {
    setup: Option<Tokens>,
    teardown: Option<Tokens>,
    tests: Vec<(String, Tokens)>
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

pub fn generate_tests_from_yaml(download_dir: &PathBuf) -> Result<(), failure::Error> {
    let paths = fs::read_dir(download_dir).unwrap();
    for entry in paths {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    generate_tests_from_yaml(&entry.path())?;
                } else if file_type.is_file() {
                    let file_name = entry.file_name().to_string_lossy().into_owned();
                    if !file_name.ends_with(".yml") && !file_name.ends_with(".yaml") {
                        continue;
                    }

                    let yaml = fs::read_to_string(&entry.path()).unwrap();
                    let docs = YamlLoader::load_from_str(&yaml).unwrap();
                    let mut test = YamlTest::new(docs.len());

                    for doc in docs {
                        println!("{:?}", &entry.path());
                        if let Some(mut hash) = doc.into_hash() {
                            let entries = hash.entries();
                            let first = entries.into_iter().next().unwrap();
                            match (first.key(), first.get()) {
                                (Yaml::String(name), Yaml::Array(steps)) => {
                                    let tokens = read_steps(steps)?;
                                    match name.as_str() {
                                        "setup" => test.setup = Some(tokens),
                                        "teardown" => test.teardown = Some(tokens),
                                        name => {
                                            test.tests.push((name.to_owned(), tokens));
                                        }
                                    };
                                }
                                (k, v) => {
                                    return Err(failure::err_msg(format!(
                                        "{:?} and {:?} in {:?} is not a string and array",
                                        &k,
                                        &v,
                                        &entry.path()
                                    )))
                                }
                            }
                        } else {
                            return Err(failure::err_msg(format!(
                                "{:?} is not a hash",
                                &entry.path()
                            )));
                        }
                    }

                    println!("{:?}", test.tests);
                }
            }
        }
    }

    Ok(())
}

fn read_steps(steps: &Array) -> Result<Tokens, failure::Error> {
    let mut tokens = Tokens::new();
    for step in steps {
        if let Some(mut hash) = step.clone().into_hash() {
            let mut entries = hash.entries();
            let mut first = entries.next().unwrap();

            let mut key = first.key().as_str().unwrap();
            let mut value = first.get().clone();

            match (key, value) {
                ("skip", Yaml::Hash(h)) => {}
                ("do", Yaml::Hash(ref mut h)) => {
                    read_do(h, &mut tokens)?;
                }
                ("set", Yaml::Hash(h)) => {}
                ("transform_and_set", Yaml::Hash(h)) => {}
                ("match", Yaml::Hash(h)) => {}
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

fn read_do(hash: &mut Hash, tokens: &mut Tokens) -> Result<(), failure::Error> {

    let mut entries = hash.entries();

    let headers = String::from("headers");
    let catch = String::from("catch");
    let node_selector = String::from("node_selector");

    let result: Result<Vec<_>, _> = entries
        .into_iter()
        .map(|entry| {
            match (entry.key(), entry.get()) {
                (Yaml::String(headers), Yaml::Hash(h)) => Ok(()),
                (Yaml::String(catch), Yaml::String(s)) => Ok(()),
                (Yaml::String(node_selector), _) => Ok(()),
                (Yaml::String(s), Yaml::Hash(h)) => {
                    let fn_name = s.clone().replace(".", "().");
                    let fn_name_ident = syn::Ident::from(fn_name);

                    tokens.append(quote! {
                        let response = client.#fn_name_ident().await?;
                    });
                    Ok(())
                },
                (k, v) => Err(failure::err_msg(format!("{:?} and {:?} are not a string and hash", &k, &v))),
            }
        })
        .collect();

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}