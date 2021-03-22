use crate::db::Db;
use crate::file::data::File;

use rocket::http::Status;
use std::path::PathBuf;

#[get("/<path..>?<host>")]
pub async fn handler(_pool: Db<'_>, host: String, path: Option<PathBuf>) -> Result<File, Status> {
    println!("{}, {:?}", host, path);
    Err(Status::NotImplemented)
}
