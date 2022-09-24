#[macro_use]
extern crate rocket;

pub mod auth;
pub mod branch;
pub mod cli;
pub mod config;
pub mod deployment;
pub mod error;
pub mod file;
pub mod migration;
pub mod serve;
mod server;
pub mod site;

use error::ApiError;
use rocket::serde::json::Json;

pub use server::start;

pub type ApiResult<T> = Result<Json<T>, ApiError>;
