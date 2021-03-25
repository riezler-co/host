use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NewBranch {
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, Serialize)]
pub struct DeploymentConfig {
    pub entrypoint: Option<String>,
    pub fallback: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct NewDeployment {
    pub site: String,
    pub branch: String,
    pub config: DeploymentConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewFile {
    pub path: String,
    pub content: Vec<u8>,
    pub size: i32,
    pub extension: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewSite {
    pub name: String,
    pub slug: String,
}
