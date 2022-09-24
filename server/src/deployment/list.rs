use crate::auth::Auth;
use crate::deployment::data::{Deployment, PartialDeployment};
use crate::error::ApiError;
use crate::ApiResult;

use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use werkbank::rocket::Db;

#[get("/list?<branch>")]
pub async fn handler(pool: Db, branch: Uuid, _auth: Auth) -> ApiResult<Vec<PartialDeployment>> {
    Deployment::list(&pool, &branch)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
