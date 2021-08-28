use crate::branch::data::Branch;
use crate::db::Db;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[get("/list?<site>")]
pub async fn handler(pool: Db<'_>, site: Uuid) -> Result<Json<Vec<Branch>>, Status> {
    Branch::list(pool.inner(), &site)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
