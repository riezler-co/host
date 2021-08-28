use crate::db::Db;
use crate::deployment::data::{Deployment, PartialDeployment};

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::uuid::Uuid;

#[get("/get?<deployment>")]
pub async fn handler(
    pool: Db<'_>,
    deployment: Uuid,
) -> Result<Json<Option<PartialDeployment>>, Status> {
    Deployment::get(pool.inner(), &deployment)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
