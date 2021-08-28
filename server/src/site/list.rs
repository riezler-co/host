use crate::db::Db;
use crate::site::data::Site;

use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/list")]
pub async fn handler(pool: Db<'_>) -> Result<Json<Vec<Site>>, Status> {
    Site::list(pool.inner())
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
