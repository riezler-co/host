use crate::branch::data::Branch;
use crate::db::Db;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[get("/get?<branch>")]
pub async fn handler(pool: Db<'_>, branch: Uuid) -> Result<Json<Option<Branch>>, Status> {
    Branch::get(pool.inner(), &branch)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
