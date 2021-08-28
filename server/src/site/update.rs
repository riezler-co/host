use crate::db::Db;
use crate::site::data::{NewSite, Site};

use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateSite {
    id: Uuid,
    data: NewSite,
}

#[post("/update", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<UpdateSite>) -> Result<Json<Site>, Status> {
    Site::update(pool.inner(), &body.id, &body.data)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
