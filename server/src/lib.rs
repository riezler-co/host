#[macro_use]
extern crate rocket;

pub mod branch;
pub mod config;
pub mod cors;
pub mod db;
pub mod deployment;
pub mod file;
pub mod serve;
mod server;
pub mod site;

pub use server::start;
