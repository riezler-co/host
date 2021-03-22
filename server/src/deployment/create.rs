use crate::branch::data::Branch;
use crate::db::Db;
use crate::deployment::data::Deployment;
use types::{NewBranch, NewDeployment};

use rocket::http::Status;
use rocket_contrib::json::Json;
use slug::slugify;

#[post("/create", data = "<body>")]
pub async fn handler(pool: Db<'_>, body: Json<NewDeployment>) -> Result<Json<Deployment>, Status> {
    let deployment = body.into_inner();

    let site = slugify(&deployment.site);
    let branch = NewBranch {
        name: deployment.branch.clone(),
        slug: slugify(&deployment.branch),
    };

    let branch = Branch::create_deployment(pool.inner(), &site, branch)
        .await
        .map_err(|_| Status::InternalServerError)?;

    Deployment::create(pool.inner(), &branch.id, deployment.config)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
