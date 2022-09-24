use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use server;
use server::{cli, migration};
use werkbank::clap::{get_config_dir, run_migration, run_server};
use werkbank::otel;

#[rocket::main]
async fn main() {
    let matches = cli::get_matches();
    let file = get_config_dir(matches.get_one::<String>("config"));
    let figment = Figment::new()
        .merge(Toml::file(file).nested())
        .merge(Env::prefixed("HOSTING_").global());

    if run_migration(&matches) {
        migration::run(&figment).await;
    }

    if let Some(_) = run_server(&matches) {
        otel::init("vulpo_host_server", &figment);
        server::start(&figment).await;
    }
}
