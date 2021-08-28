use crate::db::Db;
use crate::deployment::data::Deployment;

use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Payload {
    deployment: Uuid,
}

#[post("/delete", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<(), Status> {
    Deployment::delete(pool.inner(), &body.deployment)
        .await
        .map_err(|_| Status::InternalServerError)
}
