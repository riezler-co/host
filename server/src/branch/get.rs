use crate::branch::data::Branch;
use crate::db::Db;

use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

#[get("/get?<branch>")]
pub async fn handler(pool: Db<'_>, branch: Uuid) -> Result<Json<Option<Branch>>, Status> {
    let branch = branch.into_inner();

    Branch::get(pool.inner(), &branch)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
