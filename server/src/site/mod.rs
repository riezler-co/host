mod create;
pub mod data;
mod delete;
mod get;
mod list;
mod update;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        get::handler,
        create::handler,
        update::handler,
        delete::handler,
        list::handler,
    ]
}
