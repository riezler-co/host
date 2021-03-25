use clap::App;
use path_slash::PathExt;
use reqwest;
use server::deployment::CompletePayload;
use server::{deployment::data::Deployment, file::Payload};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use types::{DeploymentConfig, NewDeployment, NewFile};
use walkdir::WalkDir;

pub async fn deploy() {
    let client = reqwest::Client::new();

    let new_deployemnt = NewDeployment {
        site: String::from("fuuu"),
        branch: String::from("production"),
        config: DeploymentConfig {
            fallback: None,
            entrypoint: None,
        },
    };

    let res = client
        .post("http://127.0.0.1:8001/deployment/create")
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
        panic!("Something went wrong");
    }

    let deployment = match res.json::<Deployment>().await {
        Ok(d) => d,
        Err(err) => {
            println!("Mismatched type, expected Deployment");
            println!("{:?}", err);
            panic!(err);
        }
    };

    for entry in WalkDir::new("./") {
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

        let extension = Path::new(&os_path)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap();

        let mut path = os_path.to_slash().unwrap();

        if path.starts_with(".") {
            path.remove(0);
        }

        if path.starts_with("/") {
            path.remove(0);
        }

        let content = fs::read(&os_path).unwrap();
        let size = content.len() as i32;

        let file = NewFile {
            content,
            size,
            extension: extension.to_string(),
            path,
        };

        let payload = Payload {
            deployment: deployment.id.clone(),
            file,
        };

        println!("Upload File {}", payload.file.path);

        let res = client
            .post("http://127.0.0.1:8001/file/upload")
            .json(&payload)
            .send()
            .await;

        if let Err(err) = res {
            println!("Faild to upload file {}", payload.file.path);
            println!("Error: {:?}", err);
        }
    }

    let complete = CompletePayload {
        deployment: deployment.id,
    };

    let res = client
        .post("http://127.0.0.1:8001/deployment/complete")
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
}
