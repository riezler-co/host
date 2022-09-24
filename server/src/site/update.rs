use crate::auth::Auth;
use crate::error::ApiError;
use crate::site::data::{NewSite, Site};
use crate::ApiResult;

use rocket::serde::json::Json;
use serde::Deserialize;
use uuid::Uuid;
use werkbank::rocket::Db;

#[derive(Deserialize)]
pub struct UpdateSite {
    id: Uuid,
    data: NewSite,
}

#[post("/update", data = "<body>")]
pub async fn handler(pool: Db, body: Json<UpdateSite>, _auth: Auth) -> ApiResult<Site> {
    Site::update(&pool, &body.id, &body.data)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
