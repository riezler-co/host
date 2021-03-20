mod auth;
mod cors;

use crate::auth::Auth;
use crate::cors::CORS;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index(_auth: Auth) -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::ignite()
        .attach(CORS())
        .attach(Auth::fairing("http://127.0.0.1:8000/keys".to_string()))
        .mount("/", routes![index])
        .launch()
        .await;
}
