use crate::branch::data::Branch;
use crate::db::Db;

use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Payload {
    branch: Uuid,
    version: i16,
}

#[derive(Serialize)]
pub struct Response {
    version: i16,
}

#[post("/set_version", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<Json<Response>, Status> {
    Branch::set_version(pool.inner(), &body.branch, body.version)
        .await
        .map(|version| Response { version })
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
