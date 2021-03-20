use chrono::{DateTime, Utc};
use jsonwebtoken as jwt;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use reqwest;
use rocket::fairing::{AdHoc, Fairing};
use rocket::http::Status;
use rocket::request::Outcome;
use rocket::request::{FromRequest, Request};
use serde::Deserialize;
use std::collections::HashMap;
use std::mem;
use std::sync::Mutex;
use uuid::Uuid;

type Key = Vec<u8>;

#[derive(Debug)]
pub struct AuthKeys {
    pub keys: Mutex<HashMap<Uuid, Key>>,
    pub expire_at: Mutex<DateTime<Utc>>,
    pub url: String,
}

impl AuthKeys {
    pub async fn get_keys(url: &str) -> Result<AuthKeys, String> {
        let res = match reqwest::get(url).await {
            Err(_) => return Err("Failed to get keys".to_string()),
            Ok(res) => res,
        };

        let res = match res.json::<PublicKeys>().await {
            Err(_) => return Err("Invalid Key payload".to_string()),
            Ok(json) => json,
        };

        let mut keys = HashMap::new();
        res.keys.iter().for_each(|key| {
            keys.insert(key.id, key.key.to_owned());
        });

        let expire_at = Mutex::new(res.expire_at);
        let keys = Mutex::new(keys);

        Ok(AuthKeys {
            expire_at,
            keys,
            url: String::from(url),
        })
    }

    pub async fn key(&self, id: &Uuid) -> Option<Key> {
        let expired = match self.expire_at.lock() {
            Err(_) => return None,
            Ok(exp) => *exp >= Utc::now(),
        };

        if expired {
            let new_keys = AuthKeys::get_keys(&self.url).await.unwrap();

            let mut exp = self.expire_at.lock().unwrap();
            let _ = mem::replace(&mut exp, new_keys.expire_at.lock().unwrap());

            let mut keys = self.keys.lock().unwrap();
            keys.clear();
            new_keys
                .keys
                .lock()
                .unwrap()
                .iter()
                .for_each(|(id, entry)| {
                    keys.insert(*id, entry.to_vec());
                });
        }

        let keys = match self.keys.lock() {
            Err(_) => return None,
            Ok(keys) => keys,
        };

        match keys.get(&id) {
            None => None,
            Some(key) => Some(key.to_vec()),
        }
    }
}

#[derive(Debug, Deserialize)]
struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iss: Uuid,
    pub traits: Vec<String>,
}

pub struct Auth(Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token_string = match req.headers().get_one("Authorization") {
            None => return Outcome::Failure((Status::BadRequest, ())),
            Some(token) => token,
        };

        let end = token_string.len();
        let start = "Bearer ".len();
        let token = &token_string[start..end];
        let token_data = match jwt::dangerous_insecure_decode::<Claims>(&token) {
            Err(_) => return Outcome::Failure((Status::BadRequest, ())),
            Ok(td) => td,
        };

        let auth = match req.managed_state::<AuthKeys>() {
            None => return Outcome::Failure((Status::InternalServerError, ())),
            Some(auth) => auth,
        };

        let key = match auth.key(&token_data.claims.iss).await {
            None => return Outcome::Failure((Status::BadRequest, ())),
            Some(key) => key,
        };

        let decoding_key = match DecodingKey::from_rsa_pem(&key) {
            Err(_) => return Outcome::Failure((Status::BadRequest, ())),
            Ok(dk) => dk,
        };

        let token_data = match jwt::decode::<Claims>(
            &token,
            &decoding_key,
            &Validation::new(Algorithm::RS256),
        ) {
            Err(_) => return Outcome::Failure((Status::BadRequest, ())),
            Ok(td) => td,
        };

        Outcome::Success(Auth(token_data.claims))
    }
}

impl Auth {
    pub fn fairing(key_url: String) -> impl Fairing {
        AdHoc::on_attach("Get PublicKeys", move |rocket| async move {
            let auth = match AuthKeys::get_keys(&key_url).await {
                Err(msg) => {
                    println!("{}", msg);
                    return Err(rocket);
                }
                Ok(auth) => auth,
            };

            Ok(rocket.manage(auth))
        })
    }
}

#[derive(Deserialize)]
pub struct PublicKeys {
    pub expire_at: DateTime<Utc>,
    pub keys: Vec<PublicKey>,
}

#[derive(Deserialize)]
pub struct PublicKey {
    pub id: Uuid,
    pub key: Vec<u8>,
}
