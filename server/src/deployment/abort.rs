use crate::auth::Auth;
use crate::deployment::data::Deployment;
use crate::error::ApiError;
use crate::ApiResult;

use rocket::serde::json::Json;
use serde::Deserialize;
use uuid::Uuid;
use werkbank::rocket::Db;

#[derive(Deserialize)]
pub struct Payload {
    deployment: Uuid,
}

#[post("/abort", data = "<body>")]
pub async fn handler(pool: Db, body: Json<Payload>, _auth: Auth) -> ApiResult<()> {
    Deployment::abort(&pool, &body.deployment)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
