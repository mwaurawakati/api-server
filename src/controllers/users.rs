use super::generic_response;
use crate::{
    db::SqliteBackend,
    models::{NewUser, UpdateUser},
    secure::guards::ApiKeyGuard,
};
use rocket::{
    http::Status,
    serde::json::{Json, Value},
    State,
};

/// # Create a new user
///
/// This endpoint is used to create a new user.
///
/// # Requires
///
/// `X-API-KEY`: must be passed in the header with a valid API Key
///
/// # Parameters
///
/// - `user`: A JSON object containing the new user data.
///
/// # Returns
///
/// A tuple consisting of:
/// - `Status`: The HTTP status code.
/// - `Value`: The response body as a JSON value.
#[openapi(tag = "Users")]
#[post("/users", format = "json", data = "<user>")]
pub async fn create_user_endpoint(
    user: Json<NewUser>,
    backend: &State<SqliteBackend>,
    salt: &State<String>,
    _api_guard: ApiKeyGuard,
) -> (Status, Value) {
    generic_response(backend.create_user(user.into_inner(), salt).await)
}

/// # Update an existing user
///
/// This endpoint updates user information with new data.
///
/// # Requires
///
/// `X-API-KEY`: must be passed in the header with a valid API Key
///
/// # Parameters
/// - `user_id`: The ID of the user to update.
/// - `user`: The JSON representation of the user to update.
///
/// # Returns
/// A tuple containing the HTTP status code and the JSON value of the response body.
#[openapi(tag = "Users")]
#[put("/users/<user_id>", format = "json", data = "<user>")]
pub async fn update_user_endpoint(
    user_id: &str,
    user: Json<UpdateUser>,
    backend: &State<SqliteBackend>,
    salt: &State<String>,
    _api_guard: ApiKeyGuard,
) -> (Status, Value) {
    generic_response(backend.update_user(user.into_inner(), user_id, salt).await)
}

/// # Delete an existing user with the specified ID
///
/// This endpoint deletes a user by a given user id.
///
/// # Requires
///
/// `X-API-KEY`: must be passed in the header with a valid API Key
///
/// # Parameters
///
/// - `user_id`: The ID of the user to delete.
///
/// # Returns
///
/// Returns `Result` indicating success or failure of the deletion operation. If the user is
/// deleted successfully, the function returns `Ok(Status::NoContent)`. If there is an error
/// during deletion, it returns `Err` with a String containing the error message.
#[openapi(tag = "Users")]
#[delete("/users/<user_id>")]
pub async fn delete_user_endpoint(
    user_id: String,
    backend: &State<SqliteBackend>,
    _api_guard: ApiKeyGuard,
) -> (Status, Value) {
    generic_response(backend.delete_user(&user_id).await)
}

/// # List all Users
///
/// This endpoint lists all the users.
///
/// # Requires
///
/// `X-API-KEY`: must be passed in the header with a valid API Key
///
/// # Parameters
///
/// `NONE`
///
/// ## Returns
///
/// Returns a tuple containing the HTTP status and a `Value` representing the response payload.
#[openapi(tag = "Users")]
#[get("/users")]
pub async fn list_all_users_endpoint(
    backend: &State<SqliteBackend>,
    _api_guard: ApiKeyGuard,
) -> (Status, Value) {
    generic_response(backend.get_all_users().await)
}

/// # Get user information by ID
///
/// This endpoint gets user information by a given user id.
///
/// # Requires
///
/// `X-API-KEY`: must be passed in the header with a valid API Key
///
/// # Parameters
///
/// * `user_id` - The ID of the user to retrieve.
///
/// # Returns
/// Returns a tuple with a `Status` and a `Value`.
/// The `Status` represents the HTTP status code of the response.
/// The `Value` contains the user information in JSON format.
#[openapi(tag = "Users")]
#[get("/users/<user_id>")]
pub async fn get_user_by_id_endpoint(
    user_id: String,
    backend: &State<SqliteBackend>,
    _api_guard: ApiKeyGuard,
) -> (Status, Value) {
    generic_response(backend.get_user_with_id(&user_id).await)
}

/// # Get a user by their API key
///
/// This endpoint gets user information by a given api key.
///
/// # Requires
///
/// `X-API-KEY`: must be passed in the header with a valid API Key
///
/// # Parameters
/// - `NONE`
///
/// # Returns
/// A tuple containing the HTTP status and the user information in JSON format.
#[openapi(tag = "Users")]
#[get("/users/apikey")]
pub async fn get_user_by_api_key_endpoint(
    api_key: ApiKeyGuard,
    backend: &State<SqliteBackend>,
) -> (Status, Value) {
    generic_response(backend.get_user_with_apikey(&api_key.0.api_key).await)
}
