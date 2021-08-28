use crate::db::Db;
use crate::deployment::data::{Deployment, PartialDeployment};

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[get("/list?<branch>")]
pub async fn handler(pool: Db<'_>, branch: Uuid) -> Result<Json<Vec<PartialDeployment>>, Status> {
    Deployment::list(pool.inner(), &branch)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
