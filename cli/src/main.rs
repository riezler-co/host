mod config;
mod deploy;
mod site;

use crate::config::Config;
use clap::{App, Arg};
use std::path::Path;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let matches = App::new("Auth")
        .version("1.0")
        .author("Michael Riezler. <michael@riezler.co>")
        .subcommand(site::app())
        .subcommand(deploy::app())
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .takes_value(true)
                .default_value("hosting.toml")
                .global(true),
        )
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .default_value(".")
                .takes_value(true)
                .required(false)
                .global(true),
        )
        .arg(
            Arg::new("profile")
                .long("profile")
                .short('p')
                .takes_value(true)
                .default_value("default")
                .global(true),
        )
        .get_matches();

    let profile = matches.value_of("profile").unwrap();

    let root = matches.value_of("dir").unwrap();
    let config_path = Path::new(&root).join(matches.value_of("config").unwrap());
    let config_path = config_path.as_path().to_str().expect("invalid path");

    println!("CONFIG: {:?}", config_path);

    let config = match Config::new(config_path, &profile) {
        Ok(config) => config,
        Err(err) => {
            println!("Error: Config");
            panic!(err);
        }
    };

    println!("{:?}", config);

    if let Some(site) = matches.subcommand_matches("site") {
        if let Some(subcommand) = site.subcommand() {
            match subcommand {
                ("create", args) => {
                    let name = args.value_of("name").unwrap();
                    let slug = args.value_of("slug");
                    match site::create(name, slug).await {
                        Err(_) => println!("Something went wrong"),
                        Ok(_) => println!("Site Created"),
                    };
                }
                ("get", _) => {
                    match site::get().await {
                        Err(_) => println!("Something went wrong"),
                        Ok(site) => println!("{:?}", site),
                    };
                }
                ("list", _) => {
                    match site::list().await {
                        Err(_) => println!("Something went wrong"),
                        Ok(sites) => println!("{:?}", sites),
                    };
                }
                _ => {}
            }
        };
    };

    if let Some(args) = matches.subcommand_matches("deploy") {
        let site = args.value_of("site").unwrap();
        let branch = args.value_of("branch").unwrap();
        deploy::deploy(config, root, site, branch).await;
    };

    Ok(())
}
