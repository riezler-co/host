use crate::branch::data::Branch;
use crate::db::Db;

use rocket::http::Status;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

#[get("/list?<site>")]
pub async fn handler(pool: Db<'_>, site: Uuid) -> Result<Json<Vec<Branch>>, Status> {
    let site = site.into_inner();

    Branch::list(pool.inner(), &site)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
