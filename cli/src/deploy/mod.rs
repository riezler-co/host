use crate::config::Config;

use clap::{App, Arg};
use globset::{Glob, GlobSetBuilder};
use path_slash::PathExt;
use reqwest;
use server::deployment::CompletePayload;
use server::{deployment::data::Deployment, file::Payload};
use server::{
    deployment::data::{DeploymentConfig, Fallbacks, NewDeployment},
    file::data::NewFile,
};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub async fn deploy(config: Config, root: &str, site: &str, branch: &str) {
    let server = config.server();
    let client = reqwest::Client::new();

    let mut ignore = GlobSetBuilder::new();

    config.ignore.iter().for_each(|pattern| {
        let glob = Glob::new(&pattern).expect(&format!("Invald Glob Pattern: {}", pattern));
        ignore.add(glob);
    });

    let ignore = ignore.build().unwrap();

    let new_deployemnt = NewDeployment {
        site: site.to_string(),
        branch: branch.to_string(),
        config: DeploymentConfig {
            fallbacks: Fallbacks {
                not_found: config.fallback.not_found,
                spa: config.fallback.spa,
            },
            clean_urls: config.clean_urls,
            spa: config.spa,
        },
    };

    let res = client
        .post(&format!("{}/deployment/create", server))
        .json(&new_deployemnt)
        .send()
        .await;

    let res = match res {
        Ok(json) => json,
        Err(err) => {
            println!("Failed to create deployment");
            println!("{:?}", err);
            panic!(err);
        }
    };

    if res.status().is_server_error() {
        println!("Something went wrong");
        panic!(res);
    }

    let deployment = match res.json::<Deployment>().await {
        Ok(d) => d,
        Err(err) => {
            println!("Mismatched type, expected Deployment");
            println!("{:?}", err);
            panic!(err);
        }
    };

    let dir = Path::new(root).join(&config.path).join(&config.public);

    let root_path = dir.clone();

    for entry in WalkDir::new(dir) {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                println!("{:?}", err);
                panic!("Entry Error");
            }
        };

        let os_path = entry.path();
        let path = Path::new(&os_path);

        if !path.is_file() {
            continue;
        }

        if ignore.is_match(&path) {
            continue;
        }

        let extension = Path::new(&os_path)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        let path = match os_path.strip_prefix(&root_path) {
            Ok(path) => path,
            Err(err) => {
                println!("Error: Path");
                panic!(err)
            }
        };

        let content = fs::read(&os_path).unwrap();
        let size = content.len() as i32;

        let file = NewFile {
            content,
            size,
            extension: extension.to_string(),
            path: path.to_slash().unwrap(),
        };

        let payload = Payload {
            deployment: deployment.id.clone(),
            file,
        };

        println!("Upload File {}", payload.file.path);

        let res = client
            .post(&format!("{}/file/upload", server))
            .json(&payload)
            .send()
            .await;

        if let Err(err) = res {
            println!("Faild to upload file {}", payload.file.path);
            panic!(err);
        }
    }

    let complete = CompletePayload {
        deployment: deployment.id,
    };

    let res = client
        .post(&format!("{}/deployment/complete", server))
        .json(&complete)
        .send()
        .await;

    if let Err(err) = res {
        println!("Failed to complete deployment");
        println!("Error: {:?}", err);
    }
}

pub fn app() -> App<'static> {
    App::new("deploy")
        .arg(
            Arg::new("site")
                .long("site")
                .short('s')
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("branch")
                .long("branch")
                .short('b')
                .takes_value(true)
                .required(true),
        )
}
