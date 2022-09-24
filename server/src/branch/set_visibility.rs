use crate::auth::Auth;
use crate::branch::data::Branch;
use crate::error::ApiError;
use crate::ApiResult;

use rocket::serde::json::Json;
use serde::Deserialize;
use uuid::Uuid;
use werkbank::rocket::Db;

#[derive(Deserialize)]
pub struct Payload {
    branch: Uuid,
    visibility: bool,
}

#[post("/set_visibility", data = "<body>")]
pub async fn handler(pool: Db, body: Json<Payload>, _auth: Auth) -> ApiResult<()> {
    Branch::set_visibility(&pool, &body.branch, body.visibility)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
