use crate::auth::Auth;
use crate::error::ApiError;
use crate::file::data::{File, NewFile};
use crate::ApiResult;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use werkbank::rocket::Db;

#[derive(Deserialize, Serialize, Debug)]
pub struct Payload {
    pub deployment: Uuid,
    pub file: NewFile,
}

#[derive(Serialize)]
pub struct Response {
    file_id: Uuid,
}

#[post("/upload", data = "<body>")]
pub async fn handler(pool: Db, body: Json<Payload>, _auth: Auth) -> ApiResult<Response> {
    File::create(&pool, &body.deployment, &body.file)
        .await
        .map(|file_id| Response { file_id })
        .map(Json)
        .map_err(ApiError::from)
}
