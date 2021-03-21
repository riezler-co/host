use crate::branch::data::{Branch, NewBranch};
use crate::db::Db;

use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Payload {
    branch: Uuid,
    data: NewBranch,
}

#[post("/update", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<(), Status> {
    let Payload { data, branch } = body.into_inner();

    Branch::update(pool.inner(), &branch, data)
        .await
        .map_err(|_| Status::InternalServerError)
}
