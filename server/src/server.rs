use figment::{providers::Env, Figment};
use rocket::fairing::AdHoc;
use werkbank::rocket::tracing::TracingFairing;
use werkbank::rocket::{db, Cors};

use crate::branch;
use crate::config::AuthConfig;
use crate::deployment;
use crate::file;
use crate::serve;
use crate::site;

pub async fn start(config: &Figment) {
    let rocket_config = Figment::from(rocket::Config::default())
        .merge(config.clone().select("server"))
        .merge(Env::prefixed("VULPO_SERVER_").global());

    let auth_config = config
        .clone()
        .select("auth")
        .extract::<AuthConfig>()
        .expect("Auth config");

    let _ = rocket::custom(rocket_config)
        .attach(TracingFairing)
        .attach(Cors::from_figment(&config))
        .attach(AdHoc::on_ignite("Add Host Config", |rocket| async move {
            rocket.manage(auth_config)
        }))
        .attach(db::create_pool(&config))
        .mount("/site", site::routes())
        .mount("/branch", branch::routes())
        .mount("/deployment", deployment::routes())
        .mount("/file", file::routes())
        .mount("/serve", serve::routes())
        .launch()
        .await;
}
