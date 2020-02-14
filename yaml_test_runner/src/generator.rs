use quote::Tokens;
use std::fs;
use std::path::PathBuf;
use yaml_rust::{
    yaml::{Array, Hash},
    Yaml, YamlLoader,
};

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
                    let mut setup: Option<Tokens> = None;
                    let mut teardown: Option<Tokens> = None;
                    let mut tests: Vec<(String, Tokens)> = Vec::with_capacity(docs.len());

                    for doc in docs {
                        println!("{:?}", &entry.path());
                        if let Some(mut hash) = doc.into_hash() {
                            let entries = hash.entries();
                            let first = entries.into_iter().next().unwrap();
                            match (first.key(), first.get()) {
                                (Yaml::String(name), Yaml::Array(steps)) => {
                                    let tokens = read_steps(steps)?;
                                    match name.as_str() {
                                        "setup" => setup = Some(tokens),
                                        "teardown" => teardown = Some(tokens),
                                        name => {
                                            tests.push((name.to_owned(), tokens));
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
            let first = entries.next().unwrap();

            match (first.key().as_str().unwrap(), first.get()) {
                ("skip", Yaml::Hash(h)) => {}
                ("do", Yaml::Hash(h)) => {
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

fn read_do(hash: &Hash, tokens: &mut Tokens) -> Result<(), failure::Error> {
    Ok(())
}
