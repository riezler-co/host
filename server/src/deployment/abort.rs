use crate::db::Db;
use crate::deployment::data::Deployment;

use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Payload {
    deployment: Uuid,
}

#[post("/abort", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<(), Status> {
    let Payload { deployment } = body.into_inner();

    Deployment::abort(pool.inner(), &deployment)
        .await
        .map_err(|_| Status::InternalServerError)
}
