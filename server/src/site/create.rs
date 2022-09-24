use crate::auth::Auth;
use crate::error::ApiError;
use crate::site::data::{NewSite, Site};
use crate::ApiResult;

use rocket::serde::json::Json;
use werkbank::rocket::Db;

#[post("/create", data = "<body>")]
pub async fn handler(pool: Db, body: Json<NewSite>, _auth: Auth) -> ApiResult<Site> {
    Site::create(&pool, &body.into_inner())
        .await
        .map(Json)
        .map_err(ApiError::from)
}
