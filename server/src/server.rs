use crate::branch;
use crate::config::DbConfig;
use crate::cors::CORS;
use crate::db;
use crate::deployment;
use crate::file;
use crate::serve;
use crate::site;

pub async fn start(db_config: &DbConfig) {
    let _ = rocket::build()
        .attach(CORS())
        .attach(db::create_pool(db_config))
        .mount("/site", site::routes())
        .mount("/branch", branch::routes())
        .mount("/deployment", deployment::routes())
        .mount("/file", file::routes())
        .mount("/serve", serve::routes())
        .launch()
        .await;
}
