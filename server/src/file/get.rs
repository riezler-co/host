use crate::db::Db;
use crate::file::data::File;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[get("/get?<file>")]
pub async fn handler(pool: Db<'_>, file: Uuid) -> Result<Json<Option<File>>, Status> {
    File::get(pool.inner(), &file)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
