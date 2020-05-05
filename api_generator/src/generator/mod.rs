use crate::generator::code_gen::url::url_builder::PathString;
use rustfmt_nightly::{Config, Edition, EmitMode, Input, Session};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashSet},
    fmt,
    fs::{self, File, OpenOptions},
    hash::{Hash, Hasher},
    io::{prelude::*, Read},
    path::PathBuf,
};

#[cfg(test)]
use quote::{ToTokens, Tokens};
use semver::Version;
use serde::de::{Error, MapAccess, Visitor};
use std::marker::PhantomData;
use std::str::FromStr;
use void::Void;

pub mod code_gen;

/// A complete API specification parsed from the REST API specs
pub struct Api {
    pub commit: String,
    /// parameters that are common to all API methods
    pub common_params: BTreeMap<String, Type>,
    /// root API methods e.g. Search, Index
    pub root: BTreeMap<String, ApiEndpoint>,
    /// namespace client methods e.g. Indices.Create, Ml.PutJob
    pub namespaces: BTreeMap<String, BTreeMap<String, ApiEndpoint>>,
    /// enums in parameters
    pub enums: Vec<ApiEnum>,
}

impl Api {
    /// Attempt to parse the version from the commit tag, which typically
    /// will be of the form e.g. v7.6.1
    pub fn version(&self) -> Option<semver::Version> {
        let v = self.commit.trim_start_matches('v');
        semver::Version::parse(v).ok()
    }

    /// Find the right ApiEndpoint from the REST API specs for the API call
    /// defined in the YAML test.
    ///
    /// The REST API specs model only the stable APIs
    /// currently, so no endpoint will be found for experimental or beta APIs
    pub fn endpoint_for_api_call(&self, api_call: &str) -> Option<&ApiEndpoint> {
        let api_call_path: Vec<&str> = api_call.split('.').collect();
        match api_call_path.len() {
            1 => self.root.get(api_call_path[0]),
            _ => match self.namespaces.get(api_call_path[0]) {
                Some(namespace) => namespace.get(api_call_path[1]),
                None => None,
            },
        }
    }
}

/// A HTTP method in the REST API spec
#[derive(Debug, Eq, PartialEq, Deserialize, Clone, Copy, Ord, PartialOrd)]
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

/// Converts a `HttpMethod` in the REST spec, into the AST for
/// a `Method` in the elasticsearch client
impl quote::ToTokens for HttpMethod {
    fn to_tokens(&self, tokens: &mut quote::Tokens) {
        tokens.append("Method");
        tokens.append("::");
        match *self {
            HttpMethod::Head => tokens.append("Head"),
            HttpMethod::Get => tokens.append("Get"),
            HttpMethod::Post => tokens.append("Post"),
            HttpMethod::Put => tokens.append("Put"),
            HttpMethod::Patch => tokens.append("Patch"),
            HttpMethod::Delete => tokens.append("Delete"),
        }
    }
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

/// The type of the param or part
#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    None,
    List,
    Enum,
    String,
    Text,
    Boolean,
    Number,
    Float,
    Double,
    Integer,
    Long,
    Date,
    Time,
    Union(Box<(TypeKind, TypeKind)>),
}

impl<'de> Deserialize<'de> for TypeKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.contains('|') {
            let values: Vec<&str> = value.split('|').collect();

            if values.len() > 2 {
                Err(D::Error::custom(
                    "TypeKind union contains more than two values",
                ))
            } else {
                let union = Box::new((TypeKind::from(values[0]), TypeKind::from(values[1])));
                Ok(TypeKind::Union(union))
            }
        } else {
            Ok(TypeKind::from(value.as_str()))
        }
    }
}

impl From<&str> for TypeKind {
    fn from(s: &str) -> Self {
        match s {
            "list" => TypeKind::List,
            "enum" => TypeKind::Enum,
            "string" => TypeKind::String,
            "text" => TypeKind::Text,
            "boolean" => TypeKind::Boolean,
            "number" => TypeKind::Number,
            "float" => TypeKind::Float,
            "double" => TypeKind::Double,
            "int" => TypeKind::Integer,
            "long" => TypeKind::Long,
            "date" => TypeKind::Date,
            "time" => TypeKind::Time,
            n => panic!("unknown typekind {}", n),
        }
    }
}

impl Default for TypeKind {
    fn default() -> Self {
        TypeKind::None
    }
}

/// Details about a deprecated API url path
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Deprecated {
    pub version: String,
    pub description: String,
}

/// An API url path
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Path {
    pub path: PathString,
    pub methods: Vec<HttpMethod>,
    #[serde(default = "BTreeMap::new")]
    pub parts: BTreeMap<String, Type>,
    pub deprecated: Option<Deprecated>,
}

/// The URL components of an API endpoint
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Url {
    pub paths: Vec<Path>,
}

/// Body of an API endpoint
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Body {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub serialize: Option<String>,
}

lazy_static! {
    static ref VERSION: Version = semver::Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
}

/// Wraps the URL string to replace master or current in URL path with the
/// major.minor version of the api_generator.
fn documentation_url_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(DocumentationUrlString::replace_version_in_url(s))
}

/// A Documentation URL string
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct DocumentationUrlString(
    #[serde(deserialize_with = "documentation_url_string")] pub String,
);

impl DocumentationUrlString {
    fn from_url(s: String) -> Self {
        let s = Self::replace_version_in_url(s);
        Self(s)
    }

    fn replace_version_in_url(s: String) -> String {
        match url::Url::parse(&s) {
            Ok(u) => {
                let mut u = u;
                if u.path().contains("/master") {
                    u.set_path(
                        u.path()
                            .replace(
                                "/master",
                                format!("/{}.{}", VERSION.major, VERSION.minor).as_str(),
                            )
                            .as_str(),
                    );
                } else if u.path().contains("/current") {
                    u.set_path(
                        u.path()
                            .replace(
                                "/current",
                                format!("/{}.{}", VERSION.major, VERSION.minor).as_str(),
                            )
                            .as_str(),
                    );
                }
                u.into_string()
            }
            Err(_) => s,
        }
    }
}

impl core::ops::Deref for DocumentationUrlString {
    type Target = String;

    fn deref(self: &'_ Self) -> &'_ Self::Target {
        &self.0
    }
}

impl fmt::Display for DocumentationUrlString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Documentation for an API endpoint
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Documentation {
    pub url: Option<DocumentationUrlString>,
    pub description: Option<String>,
}

impl FromStr for Documentation {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Documentation {
            url: Some(DocumentationUrlString::from_url(s.to_owned())),
            description: None,
        })
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: serde::de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

/// An API endpoint defined in the REST API specs
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct ApiEndpoint {
    pub full_name: Option<String>,
    #[serde(deserialize_with = "string_or_struct")]
    documentation: Documentation,
    pub stability: String,
    pub url: Url,
    #[serde(default = "BTreeMap::new")]
    pub params: BTreeMap<String, Type>,
    pub body: Option<Body>,
}

impl ApiEndpoint {
    /// Whether the endpoint supports sending a body
    pub fn supports_body(&self) -> bool {
        self.body.is_some()
            || self.url.paths.iter().any(|p| {
                p.methods.contains(&HttpMethod::Post) || p.methods.contains(&HttpMethod::Put)
            })
    }

    /// Whether the endpoint supports sending a newline delimited body
    pub fn supports_nd_body(&self) -> bool {
        self.supports_body()
            && match &self.body {
                Some(b) => match &b.serialize {
                    Some(s) => s == "bulk",
                    _ => false,
                },
                None => false,
            }
    }
}

/// Common parameters accepted by all API endpoints
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Common {
    description: Option<String>,
    #[serde(deserialize_with = "string_or_struct")]
    documentation: Documentation,
    params: BTreeMap<String, Type>,
}

/// An enum defined in the REST API specs
pub struct ApiEnum {
    pub name: String,
    pub description: Option<String>,
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

/// Generates all client source code from the REST API spec
pub fn generate(
    branch: &str,
    download_dir: &PathBuf,
    generated_dir: &PathBuf,
) -> Result<(), failure::Error> {
    // read the Api from file
    let api = read_api(branch, download_dir)?;

    let docs_dir = {
        let d = generated_dir.clone();
        d.parent().unwrap().parent().unwrap().join("docs")
    };

    // generate param enums
    let params = code_gen::params::generate(&api)?;
    write_file(params, generated_dir, "params.rs")?;

    // generate namespace clients
    let namespace_clients = code_gen::namespace_clients::generate(&api, &docs_dir)?;
    let mut namespace_clients_dir = generated_dir.clone();
    namespace_clients_dir.push("namespace_clients");
    fs::create_dir_all(&namespace_clients_dir)?;

    // generate the mod file to reference all namespace clients
    let modules = namespace_clients
        .iter()
        .map(|(name, _)| format!("pub mod {};", name))
        .collect::<Vec<_>>()
        .join("\n");

    write_file(modules, &namespace_clients_dir, "mod.rs")?;

    for (name, input) in namespace_clients {
        write_file(
            input,
            &namespace_clients_dir,
            format!("{}.rs", name).as_str(),
        )?;
    }

    // generate functions on root of client
    let root = code_gen::root::generate(&api, &docs_dir)?;
    write_file(root, generated_dir, "root.rs")?;

    let generated_modules = fs::read_dir(generated_dir)?
        .map(|r| {
            let path = r.unwrap().path();
            format!("pub mod {};", path.file_stem().unwrap().to_string_lossy())
        })
        .collect::<Vec<_>>()
        .join("\n");

    write_file(generated_modules, &generated_dir, "mod.rs")?;

    Ok(())
}

/// Writes the input to the specified file, preceded by a header comment indicating generated code
fn write_file(input: String, dir: &PathBuf, file: &str) -> Result<(), failure::Error> {
    let mut generated_path = dir.clone();
    generated_path.push(file);
    let path = generated_path.to_string_lossy().into_owned();

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
// cargo run -p api_generator
//
// -----------------------------------------------
"
        .as_bytes(),
    )?;

    let mut file = OpenOptions::new().append(true).write(true).open(&path)?;
    file.write_all(input.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}

/// Reads Api from a directory of REST Api specs
pub fn read_api(branch: &str, download_dir: &PathBuf) -> Result<Api, failure::Error> {
    let paths = fs::read_dir(download_dir)?;
    let mut namespaces = BTreeMap::new();
    let mut enums: HashSet<ApiEnum> = HashSet::new();
    let mut common_params = BTreeMap::new();
    let root_key = "root";

    for path in paths {
        let path = path?.path();
        let name = path.file_name().map(|path| path.to_str());
        let display = path.to_string_lossy().into_owned();

        if name
            .unwrap()
            .map(|name| name.ends_with(".json") && !name.starts_with('_'))
            .unwrap_or(true)
        {
            let mut file = File::open(&path)?;
            let (name, api_endpoint) = endpoint_from_file(display, &mut file)?;

            // Only generate builders and methods for stable APIs, not experimental or beta
            if &api_endpoint.stability != "stable" {
                continue;
            }

            let name_parts: Vec<&str> = name.splitn(2, '.').collect();
            let (namespace, method_name) = match name_parts.len() {
                len if len > 1 => (name_parts[0].to_string(), name_parts[1].to_string()),
                _ => (root_key.to_string(), name),
            };

            // collect unique enum values
            for param in api_endpoint
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
                    description: param.1.description.clone(),
                    values: options,
                });
            }

            // collect api endpoints into namespaces
            if !namespaces.contains_key(&namespace) {
                let mut api_endpoints = BTreeMap::new();
                api_endpoints.insert(method_name, api_endpoint);
                namespaces.insert(namespace.to_string(), api_endpoints);
            } else {
                namespaces
                    .get_mut(&namespace)
                    .unwrap()
                    .insert(method_name, api_endpoint);
            }
        } else if name
            .map(|name| name == Some("_common.json"))
            .unwrap_or(true)
        {
            let mut file = File::open(&path)?;
            let common = common_params_from_file(display, &mut file)?;
            common_params = common.params;
        }
    }

    // extract the root methods
    let root = namespaces.remove(root_key).unwrap();

    let mut sorted_enums = enums.into_iter().collect::<Vec<_>>();
    sorted_enums.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(Api {
        commit: branch.to_string(),
        common_params,
        root,
        namespaces,
        enums: sorted_enums,
    })
}

/// deserializes an ApiEndpoint from a file
fn endpoint_from_file<R>(
    name: String,
    reader: &mut R,
) -> Result<(String, ApiEndpoint), failure::Error>
where
    R: Read,
{
    // deserialize the map from the reader
    let endpoint: BTreeMap<String, ApiEndpoint> =
        serde_json::from_reader(reader).map_err(|e| super::error::ParseError {
            message: format!("Failed to parse {} because: {}", name, e),
        })?;

    // get the first (and only) endpoint name and endpoint body
    let mut first_endpoint = endpoint.into_iter().next().unwrap();
    first_endpoint.1.full_name = Some(first_endpoint.0.clone());

    // sort the HTTP methods so that we can easily pattern match on them later
    for path in first_endpoint.1.url.paths.iter_mut() {
        path.methods.sort();
    }

    Ok(first_endpoint)
}

/// deserializes Common from a file
fn common_params_from_file<R>(name: String, reader: &mut R) -> Result<Common, failure::Error>
where
    R: Read,
{
    let common: Common = serde_json::from_reader(reader).map_err(|e| super::error::ParseError {
        message: format!("Failed to parse {} because: {}", name, e),
    })?;

    Ok(common)
}

/// formats tokens using rustfmt
/// https://github.com/bcmyers/num-format/blob/b7a99480b8087924d291887b13d8c38b7ce43a36/num-format-dev/src/rustfmt.rs
pub fn rust_fmt<S>(module: S) -> Result<String, failure::Error>
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
    let stdin = b"stdin:";
    if output.starts_with(stdin) {
        output.drain(0..stdin.len());
    }

    let s = String::from_utf8(output)?;

    // trim whitespace
    Ok(s.trim().into())
}

/// Asserts that the expected generated AST matches the actual generated AST
#[cfg(test)]
pub fn ast_eq<T: ToTokens>(expected: Tokens, actual: T) {
    assert_eq!(
        rust_fmt(expected.to_string()).unwrap(),
        rust_fmt(quote!(#actual).to_string()).unwrap()
    );
}
