use crate::db::Db;
use crate::site::data::{NewSite, Site};

use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/create", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<NewSite>) -> Result<Json<Site>, Status> {
    Site::create(pool.inner(), body.into_inner())
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
