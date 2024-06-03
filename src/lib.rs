#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_okapi;

#[macro_use]
pub(crate) mod macros;

pub mod controllers;
pub mod db;
mod error;
pub mod models;
pub mod secure;
pub mod server;

pub type Result<T> = std::result::Result<T, error::Error>;
