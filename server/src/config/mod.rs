use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Secrets {
    pub secrets_passphrase: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthConfig {
    pub api_keys: Vec<String>,
}
