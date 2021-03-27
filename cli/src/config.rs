use figment::{
    providers::{Env, Format, Toml},
    value::{Dict, Map},
    Figment, Metadata, Profile, Provider,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub public: String,
    pub path: String,
    pub ignore: Vec<String>,
    pub server: Option<String>,
    pub clean_urls: bool,
    pub spa: bool,
    pub fallback: Fallbacks,
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

impl Default for Config {
    fn default() -> Self {
        Config {
            public: "./".to_string(),
            path: "./".to_string(),
            ignore: vec![],
            server: None,
            clean_urls: true,
            spa: false,
            fallback: Fallbacks::default(),
        }
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Fallbacks {
    pub spa: String,
    pub not_found: String,
}

impl Default for Fallbacks {
    fn default() -> Self {
        Fallbacks {
            spa: String::from("index.html"),
            not_found: String::from("404.html"),
        }
    }
}
