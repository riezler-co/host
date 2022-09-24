use crate::config::Config;

use clap::{App, Arg};
use dialoguer::{theme::ColorfulTheme, Select};
use reqwest;
use server::site::data::{NewSite, Site};
use slug::slugify;

pub async fn create(
    config: &Config,
    client: &reqwest::Client,
    name: &str,
    slug: Option<&str>,
) -> Result<Site, reqwest::Error> {
    let server = config.server();

    let slug = match slug {
        None => slugify(name.clone()),
        Some(slug) => slug.to_string(),
    };

    let site = NewSite {
        name: name.to_string(),
        slug: slug.to_string(),
    };

    client
        .post(&format!("{}/site/create", server))
        .json(&site)
        .send()
        .await?
        .json::<Site>()
        .await
}

pub async fn list(config: &Config, client: &reqwest::Client) -> Result<Vec<Site>, reqwest::Error> {
    let server = config.server();

    client
        .get(&format!("{}/site/list", server))
        .send()
        .await?
        .json::<Vec<Site>>()
        .await
}

pub async fn get(config: &Config, client: &reqwest::Client) -> Result<Site, reqwest::Error> {
    let sites = list(config, client).await?;
    let selections: Vec<String> = sites.clone().iter().map(|site| site.slug.clone()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select site:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let site = sites[selection].to_owned();

    Ok(site)
}

pub fn app() -> App<'static> {
    let create = App::new("create")
        .arg(Arg::new("name").required(true))
        .arg(Arg::new("slug").required(false).long("slug"));

    let get = App::new("get");
    let list = App::new("list");

    App::new("site")
        .subcommand(create)
        .subcommand(get)
        .subcommand(list)
}
