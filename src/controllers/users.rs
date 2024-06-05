use crate::{
    db::SqliteBackend,
    models::{NewUser, UpdateUser},
};
use lambda_http::{Body, Error as LambdaError, Request, Response};

use serde_json::json;
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

pub async fn create_user_endpoint(
    backend: SqliteBackend,
    request: Request,
    salt: String,
) -> Result<Response<Body>, LambdaError> {
    let new_user: NewUser = serde_json::from_slice(request.body().as_ref())?;
    println!("{:#?}", new_user);
    match backend.create_user(new_user, &salt).await {
        Ok(o) => {
            println!("created");
            let m = json!(
            {
                "message":"user created",
                "user": o,
                "status":200
            });
            Ok(Response::builder()
                .status(201)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
        Err(e) => {
            let m = json!(
            {
                "message":"user creation failed",
                "error": e,
                "status":500
            });
            Ok(Response::builder()
                .status(500)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
    }
}

/*Ok(Response::builder()
.status(201)
.body(Body::Text("User created".into()))
.unwrap())*/

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
pub async fn update_user_endpoint(
    backend: SqliteBackend,
    request: Request,
    salt: String,
) -> Result<Response<Body>, LambdaError> {
    let path_segments: Vec<&str> = request.uri().path().split('/').collect();
    let user_id = path_segments[2];
    let update_user: UpdateUser = serde_json::from_slice(request.body().as_ref())?;
    match backend.update_user(update_user, user_id, &salt).await {
        Ok(o) => {
            let m = json!(
            {
                "message":"user updated",
                "user": o,
                "status":200
            });
            Ok(Response::builder()
                .status(201)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
        Err(e) => {
            let m = json!(
            {
                "message":"user update failed",
                "error": e,
                "status":500
            });
            Ok(Response::builder()
                .status(500)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
    }
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
pub async fn delete_user_endpoint(
    backend: SqliteBackend,
    request: Request,
) -> Result<Response<Body>, LambdaError> {
    let path_segments: Vec<&str> = request.uri().path().split('/').collect();
    let user_id = path_segments[2];
    match backend.delete_user(user_id).await {
        Ok(o) => {
            let m = json!(
            {
                "message":"user deleted",
                "user": o,
                "status":200
            });
            Ok(Response::builder()
                .status(201)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
        Err(e) => {
            let m = json!(
            {
                "message":"user delete failed",
                "error": e,
                "status":500
            });
            Ok(Response::builder()
                .status(500)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
    }
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
pub async fn list_all_users_endpoint(
    backend: SqliteBackend,
    _request: Request,
) -> Result<Response<Body>, LambdaError> {
    match backend.get_all_users().await {
        Ok(o) => {
            let m = json!(
            {
                "message":"user get list successful",
                "user": o,
                "status":200
            });
            Ok(Response::builder()
                .status(201)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
        Err(e) => {
            let m = json!(
            {
                "message":"user get list failed",
                "error": e,
                "status":500
            });
            Ok(Response::builder()
                .status(500)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
    }
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
pub async fn get_user_by_id_endpoint(
    backend: SqliteBackend,
    request: Request,
) -> Result<Response<Body>, LambdaError> {
    let path_segments: Vec<&str> = request.uri().path().split('/').collect();
    let user_id = path_segments[2];
    match backend.get_user_with_id(user_id).await {
        Ok(o) => {
            let m = json!(
            {
                "message":"get user by id successful",
                "user": o,
                "status":200
            });
            Ok(Response::builder()
                .status(201)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
        Err(e) => {
            let m = json!(
            {
                "message":"get user by id failed",
                "error": e,
                "status":500
            });
            Ok(Response::builder()
                .status(500)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
    }
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
pub async fn get_user_by_api_key_endpoint(
    backend: SqliteBackend,
    request: Request,
) -> Result<Response<Body>, LambdaError> {
    let api_key = request
        .headers()
        .get("x-api-key")
        .and_then(|header| header.to_str().ok())
        .map(|header| header.trim())
        .ok_or(crate::error::Error::UnauthenticatedUser)?;
    match backend.get_user_with_apikey(api_key).await {
        Ok(o) => {
            let m = json!(
            {
                "message":"get user by api successful",
                "user": o,
                "status":200
            });
            Ok(Response::builder()
                .status(201)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
        Err(e) => {
            let m = json!(
            {
                "message":"get user by api failed",
                "error": e,
                "status":500
            });
            Ok(Response::builder()
                .status(500)
                .body(Body::Text(m.to_string().into()))
                .unwrap())
        }
    }
}
