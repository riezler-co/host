use crate::db::Db;
use crate::site::data::Site;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[get("/get?<id>")]
pub async fn handler(pool: Db<'_>, id: Uuid) -> Result<Json<Site>, Status> {
    let entry = Site::get(pool.inner(), &id)
        .await
        .map_err(|_| Status::InternalServerError)?;

    if let Some(site) = entry {
        Ok(Json(site))
    } else {
        Err(Status::NotFound)
    }
}
