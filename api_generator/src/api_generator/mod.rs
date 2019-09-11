use rustfmt_nightly::{Config, Edition, EmitMode, Input, Session};
use serde::Deserialize;
use serde_json::Value;
use std::path::PathBuf;
use std::{
    collections::{BTreeMap, HashSet},
    fs::{read_dir, File},
    hash::{Hash, Hasher},
    io::{prelude::*, Read},
};

mod code_gen;

/// A complete API specification parsed from the REST API specs
pub struct Api {
    pub commit: String,
    pub namespaces: BTreeMap<String, BTreeMap<String, ApiEndpoint>>,
    pub enums: HashSet<ApiEnum>,
}

#[derive(Debug, PartialEq, Deserialize, Clone, Copy)]
pub enum HttpMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "DELETE")]
    Delete,
}

/// A type defined in the REST API spec
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Type {
    #[serde(rename = "type", default)]
    pub ty: TypeKind,
    pub description: Option<String>,
    #[serde(default = "Vec::new")]
    pub options: Vec<Value>,
    #[serde(default)]
    pub default: Option<Value>,
}

/// The kind of type
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub enum TypeKind {
    None,
    #[serde(rename = "list")]
    List,
    #[serde(rename = "enum")]
    Enum,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "double")]
    Double,
    #[serde(rename = "integer", alias = "int")]
    Integer,
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "time")]
    Time,
    #[serde(rename = "duration")]
    Duration,
}

impl Default for TypeKind {
    fn default() -> Self {
        TypeKind::None
    }
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Url {
    pub paths: Vec<String>,
    #[serde(default = "BTreeMap::new")]
    pub parts: BTreeMap<String, Type>,
    #[serde(default = "BTreeMap::new")]
    pub params: BTreeMap<String, Type>,
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Body {
    pub description: String,
}

/// An API endpoint defined in the REST API specs
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct ApiEndpoint {
    documentation: Option<String>,
    stability: String,
    methods: Vec<HttpMethod>,
    url: Url,
    body: Option<Body>,
}

/// An enum defined in the REST API specs
pub struct ApiEnum {
    pub name: String,
    pub values: Vec<String>,
}

impl Hash for ApiEnum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for ApiEnum {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for ApiEnum {}

pub fn generate(branch: &str, download_dir: &PathBuf, generated_dir: &PathBuf) {
    let api = read_api(branch, download_dir).unwrap();

    let enums = code_gen::enums::generate_enums(&api).unwrap();
    write_file(enums, generated_dir, "enums.rs");

    code_gen::namespace_client::generate_namespace_clients(&api, generated_dir);
}

fn write_file(input: String, dir: &PathBuf, file: &str) {
    let mut generated_path = dir.clone();
    generated_path.push(file);

    let mut file = File::create(generated_path.to_string_lossy().into_owned())
        .expect(format!("failed to create {}", file).as_str());
    file.write_all(input.as_bytes()).unwrap();
}

fn read_api(branch: &str, download_dir: &PathBuf) -> Result<Api, String> {
    let paths = read_dir(download_dir).unwrap();
    let mut namespaces: BTreeMap<String, BTreeMap<String, ApiEndpoint>> = BTreeMap::new();
    let mut enums: HashSet<ApiEnum> = HashSet::new();

    for path in paths {
        let path = path.unwrap().path();
        let name = path.file_name().map(|path| path.to_string_lossy());
        let display = path.to_string_lossy().into_owned();

        if name.map(|name| !name.starts_with("_")).unwrap_or(true) {
            let mut file = File::open(&path).unwrap();
            let (name, api_endpoint) = endpoint_from_file(display, &mut file)?;

            let name_parts: Vec<&str> = name.splitn(2, '.').collect();
            let namespace = match name_parts.len() {
                len if len > 1 => name_parts[0].to_string(),
                _ => "global".to_string(),
            };

            // collect unique enum values
            for param in api_endpoint
                .url
                .params
                .iter()
                .filter(|p| p.1.ty == TypeKind::Enum)
            {
                let options: Vec<String> = param
                    .1
                    .options
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect();
                enums.insert(ApiEnum {
                    name: param.0.to_string(),
                    values: options,
                });
            }

            // collect api endpoints into namespaces
            if !namespaces.contains_key(&namespace) {
                let mut api_endpoints = BTreeMap::new();
                api_endpoints.insert(name, api_endpoint);
                namespaces.insert(namespace.to_string(), api_endpoints);
            } else {
                namespaces
                    .get_mut(&namespace)
                    .unwrap()
                    .insert(name, api_endpoint);
            }
        }
    }

    Ok(Api {
        commit: branch.to_string(),
        namespaces,
        enums,
    })
}

fn endpoint_from_file<R>(name: String, reader: &mut R) -> Result<(String, ApiEndpoint), String>
where
    R: Read,
{
    let endpoint: BTreeMap<String, ApiEndpoint> = serde_json::from_reader(reader)
        .map_err(|e| format!("Failed to parse {} because: {}", name, e))?;

    Ok(endpoint.into_iter().next().unwrap())
}

fn rust_fmt<S>(module: S) -> Result<String, failure::Error>
where
    S: Into<String>,
{
    let input = Input::Text(module.into());
    let mut config = Config::default();
    config.set().edition(Edition::Edition2018);
    config.set().emit_mode(EmitMode::Stdout);
    let mut output = Vec::new();
    {
        let mut session = Session::new(config, Some(&mut output));
        let _format_report = session.format(input)?;
    }

    // remove stdin: from start of output
    let b = "stdin:".as_bytes();
    if output.starts_with(b) {
        output.drain(0..b.len());
    }

    let s = String::from_utf8(output)?;
    Ok(s)
}
