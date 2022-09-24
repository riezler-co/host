use crate::auth::Auth;
use crate::branch::data::Branch;
use crate::error::ApiError;
use crate::ApiResult;

use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use werkbank::rocket::Db;

#[get("/list?<site>")]
pub async fn handler(pool: Db, site: Uuid, _auth: Auth) -> ApiResult<Vec<Branch>> {
    Branch::list(&pool, &site)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
