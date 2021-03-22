use crate::db::Db;
use crate::file::data::File;
use types::NewFile;

use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Payload {
    deployment: Uuid,
    file: NewFile,
}

#[derive(Serialize)]
pub struct Response {
    file_id: Uuid,
}

#[post("/upload", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<Json<Response>, Status> {
    let Payload { deployment, file } = body.into_inner();

    File::create(pool.inner(), &deployment, file)
        .await
        .map(|file_id| Response { file_id })
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
