use std::collections::{BTreeMap, HashSet};
use std::hash::{Hash, Hasher};

pub struct ApiSpec {
    pub commit: String,
    pub endpoints: BTreeMap<String, ApiEndpoint>,
    pub enums: HashSet<ApiEnum>
}

pub struct ApiEndpoint {
    pub name: String,

}

pub struct ApiEnum {
    pub name: String,
    pub values: Vec<String>
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

pub fn generate(branch : &str, download_dir : &str) {

}