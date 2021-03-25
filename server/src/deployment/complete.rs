use crate::branch::data::Branch;
use crate::db::Db;
use crate::deployment::data::Deployment;

use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Payload {
    pub deployment: Uuid,
}

#[post("/complete", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<Json<Branch>, Status> {
    let Payload { deployment } = body.into_inner();

    Deployment::complete(pool.inner(), &deployment)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
