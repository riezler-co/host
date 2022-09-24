use crate::auth::Auth;
use crate::branch::data::{Branch, NewBranch};
use crate::deployment::data::{Deployment, NewDeployment};
use crate::error::ApiError;
use crate::ApiResult;

use rocket::serde::json::Json;
use slug::slugify;
use werkbank::rocket::Db;

#[post("/create", data = "<body>")]
pub async fn handler(pool: Db, body: Json<NewDeployment>, _auth: Auth) -> ApiResult<Deployment> {
    let deployment = body.into_inner();
    let site = slugify(&deployment.site);

    let branch = NewBranch {
        name: deployment.branch.clone(),
        slug: slugify(&deployment.branch),
    };

    let branch = Branch::create_deployment(&pool, &site, branch).await?;

    Deployment::create(&pool, &branch.id, deployment.config)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
