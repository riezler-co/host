use crate::deployment::data::Deployment;
use crate::file::data::File;

use futures::future::try_join_all;
use path_slash::PathExt;
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::request::{FromRequest, Request};
use std::path::PathBuf;
use werkbank::rocket::Db;

#[get("/<path..>")]
pub async fn handler(
    pool: Db,
    path: Option<PathBuf>,
    info: SiteInfo,
) -> Result<Option<File>, Status> {
    let mut path = path.unwrap_or_else(|| PathBuf::from("/"));

    let config = Deployment::config(&pool, &info.site, &info.branch)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let paths = if config.clean_urls && path.extension().is_none() {
        let index_file = path.join("index.html");
        let index_file = index_file.to_slash();

        path.set_extension("html");
        let file_path = path.as_path().to_slash();

        vec![file_path, index_file]
    } else {
        vec![path.to_slash()]
    };

    let files: Vec<_> = paths
        .iter()
        .flat_map(std::convert::identity)
        .map(|path| File::serve(&pool, &info.site, &info.branch, &path))
        .collect();

    let files = try_join_all(files)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let file = files
        .into_iter()
        .find(|file| file.to_owned().is_some())
        .flatten();

    if file.is_some() {
        return Ok(file);
    }

    let fallback = if config.spa == true {
        config.fallbacks.spa
    } else {
        config.fallbacks.not_found
    };

    File::serve(&pool, &info.site, &info.branch, &fallback)
        .await
        .map_err(|_| Status::InternalServerError)
}

pub struct SiteInfo {
    pub branch: String,
    pub site: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SiteInfo {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let host: Vec<&str> = match req.headers().get_one("Host") {
            None => return Outcome::Failure((Status::BadRequest, ())),
            Some(host) => host.split('.').collect(),
        };

        let branch = match host.get(0) {
            None => return Outcome::Failure((Status::BadRequest, ())),
            Some(branch) => branch,
        };

        let site = match host.get(1) {
            None => return Outcome::Failure((Status::BadRequest, ())),
            Some(site) => site,
        };

        Outcome::Success(SiteInfo {
            branch: branch.to_string(),
            site: site.to_string(),
        })
    }
}
