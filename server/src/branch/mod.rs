mod create;
pub mod data;
mod delete;
mod get;
mod list;
mod set_version;
mod set_visibility;
mod update;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![
        create::handler,
        update::handler,
        delete::handler,
        list::handler,
        set_visibility::handler,
        set_version::handler,
        get::handler,
    ]
}
