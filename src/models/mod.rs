use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Struct representing an API user.
#[derive(Serialize, Deserialize, Clone, Debug, FromRow, JsonSchema)]
pub struct User {
    pub api_key: String,
    pub user_id: String,
    pub password: String,
    pub email_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow, JsonSchema)]
pub struct UserInfo {
    pub api_key: String,
    pub user_id: String,
    pub email_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow, JsonSchema)]
pub struct NewUser {
    pub user_id: String,
    pub password: String,
    pub email_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow, JsonSchema)]
pub struct UpdateUser {
    pub password: Option<String>,
    pub api_key: Option<String>,
}
