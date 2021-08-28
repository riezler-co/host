use crate::db::Db;
use crate::site::data::Site;

use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Payload {
    id: Uuid,
}

#[post("/delete", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<Payload>) -> Result<(), Status> {
    Site::delete(pool.inner(), &body.id)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(())
}
