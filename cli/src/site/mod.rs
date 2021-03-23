use clap::{App, Arg};
use dialoguer::{theme::ColorfulTheme, Select};
use reqwest;
use server::site::data::Site;
use slug::slugify;
use types::NewSite;

pub async fn create(name: &str, slug: Option<&str>) -> Result<Site, reqwest::Error> {
    let slug = match slug {
        None => slugify(name.clone()),
        Some(slug) => slug.to_string(),
    };

    let site = NewSite {
        name: name.to_string(),
        slug: slug.to_string(),
    };

    let client = reqwest::Client::new();

    client
        .post("http://127.0.0.1:8001/site/create")
        .json(&site)
        .send()
        .await?
        .json::<Site>()
        .await
}

pub async fn list() -> Result<Vec<Site>, reqwest::Error> {
    reqwest::get("http://127.0.0.1:8001/site/list")
        .await?
        .json::<Vec<Site>>()
        .await
}

pub async fn get() -> Result<Site, reqwest::Error> {
    let sites = list().await?;
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
