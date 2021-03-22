use crate::db::Db;
use crate::file::data::File;

use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

#[get("/get?<file>")]
pub async fn handler(pool: Db<'_>, file: Uuid) -> Result<Json<Option<File>>, Status> {
    File::get(pool.inner(), &file.into_inner())
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
