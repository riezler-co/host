use crate::branch::data::Branch;
use crate::db::Db;
use crate::deployment::data::Deployment;

use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Payload {
    pub deployment: Uuid,
}

#[post("/complete", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<Json<Branch>, Status> {
    Deployment::complete(pool.inner(), &body.deployment)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
