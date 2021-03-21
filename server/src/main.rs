mod auth;
mod branch;
mod config;
mod cors;
mod db;
mod deployment;
mod site;

use crate::auth::Auth;
use crate::config::DbConfig;
use crate::cors::CORS;

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use rocket::Config;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Env::prefixed("AUTH_").global());

    let db_config = figment.clone().select("database");
    let db_config = db_config
        .extract::<DbConfig>()
        .expect("Invalid Database config");

    let _ = rocket::ignite()
        .attach(CORS())
        .attach(db::create_pool(&db_config))
        .attach(Auth::fairing("http://127.0.0.1:8000/keys".to_string()))
        .mount("/site", site::routes())
        .mount("/branch", branch::routes())
        .mount("/deployment", deployment::routes())
        .launch()
        .await;
}
