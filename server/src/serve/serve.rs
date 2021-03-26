use crate::db::Db;
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
    let path = match path {
        Some(path) => path,
        None => PathBuf::from("index.html"),
    };

    let entrypoint = PathBuf::from("build").join(path);
    let path = entrypoint.to_slash().unwrap();

    println!("PATH: {:?}", path);

    let file = File::serve(pool.inner(), &site, &branch, &path)
        .await
        .map_err(|_| Status::InternalServerError)?;

    if file.is_some() {
        return Ok(file);
    }

    File::serve(pool.inner(), &site, &branch, "build/index.html")
        .await
        .map_err(|_| Status::InternalServerError)
}
