use crate::db::Db;
use crate::deployment::data::{Deployment, PartialDeployment};

use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

#[get("/get?<deployment>")]
pub async fn handler(
    pool: Db<'_>,
    deployment: Uuid,
) -> Result<Json<Option<PartialDeployment>>, Status> {
    Deployment::get(pool.inner(), &deployment.into_inner())
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
