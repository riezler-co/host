use crate::db::Db;
use crate::deployment::data::{Deployment, PartialDeployment};

use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

#[get("/list?<branch>")]
pub async fn handler(pool: Db<'_>, branch: Uuid) -> Result<Json<Vec<PartialDeployment>>, Status> {
    Deployment::list(pool.inner(), &branch.into_inner())
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
