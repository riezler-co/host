use crate::auth::Auth;
use crate::error::ApiError;
use crate::site::data::Site;
use crate::ApiResult;

use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use werkbank::rocket::Db;

#[get("/get?<id>")]
pub async fn handler(pool: Db, id: Uuid, _auth: Auth) -> ApiResult<Site> {
    Site::get(&pool, &id)
        .await?
        .ok_or(ApiError::NotFound)
        .map(Json)
}
