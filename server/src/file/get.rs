use crate::auth::Auth;
use crate::error::ApiError;
use crate::file::data::File;
use crate::ApiResult;

use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;
use werkbank::rocket::Db;

#[get("/get?<file>")]
pub async fn handler(pool: Db, file: Uuid, _auth: Auth) -> ApiResult<Option<File>> {
    File::get(&pool, &file)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
