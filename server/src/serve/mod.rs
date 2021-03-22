mod serve;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![serve::handler]
}
