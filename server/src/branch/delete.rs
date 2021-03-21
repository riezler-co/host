use crate::branch::data::Branch;
use crate::db::Db;

use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Payload {
    branch: Uuid,
}

#[post("/delete", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<(), Status> {
    let Payload { branch } = body.into_inner();

    Branch::delete(pool.inner(), &branch)
        .await
        .map_err(|_| Status::InternalServerError)
}
