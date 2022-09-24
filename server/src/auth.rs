use rocket::http::Status;
use rocket::request::Outcome;
use rocket::request::{FromRequest, Request};

use crate::config::AuthConfig;

pub struct Auth;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let api_key = match req.headers().get_one("Authorization") {
            None => return Outcome::Failure((Status::BadRequest, ())),
            Some(token) => token,
        };

        let auth_config = match req.rocket().state::<AuthConfig>() {
            None => return Outcome::Failure((Status::InternalServerError, ())),
            Some(config) => config,
        };

        let next = auth_config.api_keys.contains(&api_key.to_string());

        if next == false {
            return Outcome::Failure((Status::Forbidden, ()));
        }

        Outcome::Success(Auth)
    }
}
