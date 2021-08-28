use crate::branch::data::Branch;
use crate::db::Db;

use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Payload {
    branch: Uuid,
    visibility: bool,
}

#[post("/set_visibility", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<(), Status> {
    Branch::set_visibility(pool.inner(), &body.branch, body.visibility)
        .await
        .map_err(|_| Status::InternalServerError)
}
