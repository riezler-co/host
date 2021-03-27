use crate::branch::data::{Branch, NewBranch};
use crate::db::Db;

use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Payload {
    site: Uuid,
    branch: NewBranch,
}

#[post("/create", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<Json<Branch>, Status> {
    let Payload { site, branch } = body.into_inner();

    Branch::create(pool.inner(), &site, branch)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
