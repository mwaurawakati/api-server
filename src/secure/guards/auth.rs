use crate::{db::SqliteBackend, error::Error, models::UserInfo, secure::guards::GuardedData};
use derive_more::Deref;
use rocket::{
    http::Status,
    outcome::try_outcome,
    request::{FromRequest, Outcome, Request},
    State,
};
use rocket_okapi::OpenApiFromRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Deref, OpenApiFromRequest)]
pub struct ApiKeyGuard(pub GuardedData<UserInfo>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKeyGuard {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let backend = try_outcome!(request
            .guard::<&State<SqliteBackend>>()
            .await
            .map_error(|_| (Status::InternalServerError, Error::InternalError)));

        match super::get_user_from_request(request, backend)
            .await
            .map(Self)
        {
            Ok(guard) => Outcome::Success(guard),
            Err(e) => Outcome::Error((e.to_status(), e)),
        }
    }
}
