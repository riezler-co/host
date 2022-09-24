use crate::auth::Auth;
use crate::branch::data::{Branch, NewBranch};
use crate::error::ApiError;
use crate::ApiResult;

use rocket::serde::json::Json;
use serde::Deserialize;
use uuid::Uuid;
use werkbank::rocket::Db;

#[derive(Deserialize)]
pub struct Payload {
    site: Uuid,
    branch: NewBranch,
}

#[post("/create", data = "<body>")]
pub async fn handler(pool: Db, body: Json<Payload>, _auth: Auth) -> ApiResult<Branch> {
    Branch::create(&pool, &body.site, &body.branch)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
