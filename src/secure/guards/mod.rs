use crate::db::SqliteBackend;
use crate::error::Error;
use crate::models::UserInfo;
use derive_more::Deref;
use lambda_http::Request;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Deref)]
pub struct GuardedData<T> {
    #[deref]
    pub inner: T,
    pub ip: IpAddr,
}

pub(crate) async fn get_user_from_request(
    request: &Request,
    backend: &SqliteBackend,
) -> Result<GuardedData<UserInfo>, Error> {
    // Extract API key from headers
    let api_key = request
        .headers()
        .get("x-api-key")
        .and_then(|header| header.to_str().ok())
        .map(|header| header.trim())
        .ok_or(Error::UnauthenticatedUser)?;

    // Retrieve user using the API key
    let user = backend.get_user_with_apikey(api_key).await?;

    // Extract client IP from request context
    let ip = get_client_ip(request)?;

    Ok(GuardedData { inner: user, ip })
}

// Function to extract client IP from the request
fn get_client_ip(request: &Request) -> Result<IpAddr, Error> {
    // Lambda HTTP stores client IP in the headers under different keys
    // You may need to check multiple headers depending on how your API Gateway is configured
    let headers = request.headers();
    if let Some(ip_str) = headers
        .get("X-Forwarded-For")
        .and_then(|value| value.to_str().ok())
    {
        if let Some(ip) = ip_str.split(',').next() {
            return ip.parse().map_err(|_| Error::ForbiddenAccess);
        }
    }

    // Fall back to another common header
    if let Some(ip_str) = headers
        .get("X-Real-IP")
        .and_then(|value| value.to_str().ok())
    {
        return ip_str.parse().map_err(|_| Error::ForbiddenAccess);
    }

    // Return error if no valid IP found
    Err(Error::ForbiddenAccess)
}
