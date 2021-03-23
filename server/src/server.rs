use crate::auth::Auth;
use crate::branch;
use crate::config::DbConfig;
use crate::cors::CORS;
use crate::db;
use crate::deployment;
use crate::file;
use crate::serve;
use crate::site;

pub async fn start(db_config: &DbConfig) {
    let _ = rocket::ignite()
        .attach(CORS())
        .attach(db::create_pool(db_config))
        .attach(Auth::fairing("http://127.0.0.1:8000/keys".to_string()))
        .mount("/site", site::routes())
        .mount("/branch", branch::routes())
        .mount("/deployment", deployment::routes())
        .mount("/file", file::routes())
        .mount("/serve", serve::routes())
        .launch()
        .await;
}
