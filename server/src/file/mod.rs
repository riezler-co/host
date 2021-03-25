pub mod data;
mod get;
mod upload;

use rocket::Route;

pub use upload::Payload;

pub fn routes() -> Vec<Route> {
    routes![upload::handler, get::handler]
}
