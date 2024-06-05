use crate::secure::guards::get_user_from_request;
use crate::{controllers, db::SqliteBackend, error::Error};
use catchers::{not_authorized, not_found};
use clap::Parser;
use lambda_http::{service_fn, Body, Error as LambdaError, Request, Response};
use std::fs::File;
use std::path::Path;
/// Server & App Configurations
pub mod config;

/// Catchers like 500, 501, 404, etc
mod catchers;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct CliOpts {
    /// loads the server configurations
    #[clap(short = 'c', long)]
    config: String,
}

//#[tokio::main]
pub async fn main1() -> Result<(), LambdaError> {
    lambda_http::run(service_fn(handler)).await?;
    Ok(())
}

async fn handler(event: Request) -> Result<Response<Body>, LambdaError> {
    let db_url = String::from("/tmp/test.db");
    if db_url.is_empty() {
        return Err(Box::new(Error::DatabaseNotConfigured));
    }

    if !Path::new(&db_url.clone()).exists() {
        File::create(db_url.clone())?;
    }
    let db_backend = SqliteBackend::new_connection(&db_url).await?;
    db_backend.check_and_create_table().await?;

    let salt = String::from("12345678");
    match get_user_from_request(&event, &db_backend).await {
        Ok(_) => {
            // Match the incoming request to the corresponding controller function
            match (event.method().as_str(), event.uri().path()) {
                ("POST", "/users") => {
                    controllers::users::create_user_endpoint(db_backend, event, salt).await
                }
                ("GET", path) if path.starts_with("/users/") => {
                    controllers::users::update_user_endpoint(db_backend, event, salt).await
                }
                ("GET", path) if path.starts_with("/users/") => {
                    controllers::users::delete_user_endpoint(db_backend, event).await
                }
                ("GET", "/users") => {
                    controllers::users::list_all_users_endpoint(db_backend, event).await
                }
                ("GET", path) if path.starts_with("/users/") => {
                    controllers::users::get_user_by_id_endpoint(db_backend, event).await
                }
                ("GET", "/users/api_key") => {
                    controllers::users::get_user_by_api_key_endpoint(db_backend, event).await
                }
                _ => Ok(not_found(&event).await),
            }
        }
        Err(_) => Ok(not_authorized(&event).await),
    }
}
