use server;
use server::config::DbConfig;

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use rocket::Config;

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

    server::start(&db_config).await;
}
