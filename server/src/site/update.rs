use crate::db::Db;
use crate::site::data::Site;
use types::NewSite;

use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateSite {
    id: Uuid,
    data: NewSite,
}

#[post("/update", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<UpdateSite>) -> Result<Json<Site>, Status> {
    let UpdateSite { id, data } = body.into_inner();

    Site::update(pool.inner(), &id, data)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
