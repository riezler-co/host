use crate::auth::Auth;
use crate::error::ApiError;
use crate::site::data::Site;
use crate::ApiResult;

use rocket::serde::json::Json;
use werkbank::rocket::Db;

#[get("/list")]
pub async fn handler(pool: Db, _auth: Auth) -> ApiResult<Vec<Site>> {
    Site::list(&pool).await.map(Json).map_err(ApiError::from)
}
