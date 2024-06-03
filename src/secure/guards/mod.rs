pub mod auth;
pub mod client;
pub(crate) mod cors;

pub use auth::ApiKeyGuard;

use crate::{db::SqliteBackend, error::Error, models::UserInfo};
use derive_more::Deref;
use rocket::request::Request;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Deref)]
pub struct GuardedData<T> {
    #[deref]
    pub inner: T,
    pub ip: IpAddr,
}

pub(crate) async fn get_user_from_request(
    request: &Request<'_>,
    backend: &SqliteBackend,
) -> Result<GuardedData<UserInfo>, Error> {
    let api_key = request
        .headers()
        .get_one("x-api-key")
        .map(|header| header.trim())
        .ok_or(Error::UnauthenticatedUser)?;

    backend
        .get_user_with_apikey(api_key)
        .await
        .and_then(|user| match (true, request.client_ip()) {
            (true, Some(ip)) => Ok(GuardedData { inner: user, ip }),
            _ => Err(Error::ForbiddenAccess),
        })
}
