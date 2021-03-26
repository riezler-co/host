use figment::{
    providers::{Env, Format, Toml},
    value::{Dict, Map},
    Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub entrypoint: String,
    pub path: String,
    pub ignore: Vec<String>,
    pub server: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            entrypoint: "./".to_string(),
            path: "./".to_string(),
            ignore: vec![],
            server: None,
        }
    }
}

impl Config {
    pub fn new(path: &str) -> Result<Self, figment::Error> {
        Figment::from(Config::default())
            .merge(Toml::file(path))
            .merge(Env::prefixed("HOSTING_"))
            .extract()
    }

    pub fn server(&self) -> String {
        self.server.clone().expect("Error: Server Config missing")
    }
}

// Make `Config` a provider itself for composability.
impl Provider for Config {
    fn metadata(&self) -> Metadata {
        Metadata::named("Hosting Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
        figment::providers::Serialized::defaults(Config::default()).data()
    }

    fn profile(&self) -> Option<Profile> {
        None
    }
}
