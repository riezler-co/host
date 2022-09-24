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
}

#[post("/delete", data = "<body>")]
pub async fn handler(pool: Db, body: Json<Payload>, _auth: Auth) -> ApiResult<()> {
    Branch::delete(&pool, &body.branch)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
