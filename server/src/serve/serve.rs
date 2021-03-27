use crate::db::Db;
use crate::deployment::data::Deployment;
use crate::file::data::File;

use path_slash::PathExt;
use rocket::http::Status;
use std::path::PathBuf;

#[get("/<path..>?<site>&<branch>")]
pub async fn handler(
    pool: Db<'_>,
    site: String,
    branch: String,
    path: Option<PathBuf>,
) -> Result<Option<File>, Status> {
    let config = match Deployment::config(pool.inner(), &site, &branch).await {
        Err(_) => return Err(Status::InternalServerError),
        Ok(config) => config,
    };

    let path = path
        .map(|path| {
            if config.clean_urls && path.extension().is_none() {
                path.join("index.html")
            } else {
                path
            }
        })
        .unwrap_or_else(|| PathBuf::from("/"));

    let path = path.to_slash().unwrap();

    println!("PATH: {:?}", path);

    let file = File::serve(pool.inner(), &site, &branch, &path)
        .await
        .map_err(|_| Status::InternalServerError)?;

    if file.is_some() {
        return Ok(file);
    }

    if config.spa == false {
        return File::serve(pool.inner(), &site, &branch, &config.fallbacks.not_found)
            .await
            .map_err(|_| Status::InternalServerError);
    }

    File::serve(pool.inner(), &site, &branch, &config.fallbacks.spa)
        .await
        .map_err(|_| Status::InternalServerError)
}
