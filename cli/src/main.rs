mod deploy;
mod site;

use clap::App;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let matches = App::new("Auth")
        .version("1.0")
        .author("Michael Riezler. <michael@riezler.co>")
        .subcommand(site::app())
        .subcommand(deploy::app())
        .get_matches();

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

    if let Some(_) = matches.subcommand_matches("deploy") {
        deploy::deploy().await;
    };

    Ok(())
}
