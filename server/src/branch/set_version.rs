use crate::branch::data::Branch;
use crate::error::ApiError;
use crate::{auth::Auth, ApiResult};

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use werkbank::rocket::Db;

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
pub async fn handler(pool: Db, body: Json<Payload>, _auth: Auth) -> ApiResult<Response> {
    Branch::set_version(&pool, &body.branch, body.version)
        .await
        .map(|version| Response { version })
        .map(Json)
        .map_err(ApiError::from)
}
