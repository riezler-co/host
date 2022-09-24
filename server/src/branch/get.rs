use crate::auth::Auth;
use crate::branch::data::Branch;
use crate::error::ApiError;
use crate::ApiResult;

use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use werkbank::rocket::Db;

#[get("/get?<branch>")]
pub async fn handler(pool: Db, branch: Uuid, _auth: Auth) -> ApiResult<Option<Branch>> {
    Branch::get(&pool, &branch)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
