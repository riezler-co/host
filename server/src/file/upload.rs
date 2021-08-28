use crate::db::Db;
use crate::file::data::{File, NewFile};

use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<Json<Response>, Status> {
    File::create(pool.inner(), &body.deployment, &body.file)
        .await
        .map(|file_id| Response { file_id })
        .map(Json)
        .map_err(|err| {
            println!("{:?}", err);
            Status::InternalServerError
        })
}
