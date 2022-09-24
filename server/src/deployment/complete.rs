use crate::branch::data::Branch;
use crate::deployment::data::Deployment;
use crate::error::ApiError;
use crate::{auth::Auth, ApiResult};

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use werkbank::rocket::Db;

#[derive(Deserialize, Serialize)]
pub struct Payload {
    pub deployment: Uuid,
}

#[post("/complete", data = "<body>")]
pub async fn handler(pool: Db, body: Json<Payload>, _auth: Auth) -> ApiResult<Branch> {
    Deployment::complete(&pool, &body.deployment)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
