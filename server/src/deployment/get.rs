use crate::auth::Auth;
use crate::deployment::data::{Deployment, PartialDeployment};
use crate::error::ApiError;
use crate::ApiResult;

use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use werkbank::rocket::Db;

#[get("/get?<deployment>")]
pub async fn handler(
    pool: Db,
    deployment: Uuid,
    _auth: Auth,
) -> ApiResult<Option<PartialDeployment>> {
    Deployment::get(&pool, &deployment)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
