mod abort;
mod complete;
mod create;
pub mod data;
mod delete;
mod get;
mod list;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        create::handler,
        delete::handler,
        abort::handler,
        complete::handler,
        list::handler,
        get::handler,
    ]
}
