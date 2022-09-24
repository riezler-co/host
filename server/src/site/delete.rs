use crate::auth::Auth;
use crate::error::ApiError;
use crate::site::data::Site;
use crate::ApiResult;

use rocket::serde::json::Json;
use serde::Deserialize;
use uuid::Uuid;
use werkbank::rocket::Db;

#[derive(Deserialize)]
pub struct Payload {
    id: Uuid,
}

#[post("/delete", data = "<body>")]
pub async fn handler(pool: Db, body: Json<Payload>, _auth: Auth) -> ApiResult<()> {
    Site::delete(&pool, &body.id)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
