pub mod data;
mod get;
mod upload;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![upload::handler, get::handler]
}
