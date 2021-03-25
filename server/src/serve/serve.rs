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
        None => PathBuf::from("/"),
    };

    let path = path.to_slash().unwrap();

    println!("PATH: {:?}", path);

    File::serve(pool.inner(), &site, &branch, &path)
        .await
        .map_err(|_| Status::InternalServerError)
}
