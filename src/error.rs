#![allow(clippy::enum_variant_names)]

use rocket::{
    http::Status,
    request::Request,
    response::{self, Responder},
    serde::json::Json,
};
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Connection pool error: {0}")]
    PoolError(#[from] sqlx::error::Error),
    #[error("Format error: {0}")]
    FormatError(String),
    #[error("Launch failed: {0}")]
    RocketError(#[from] Box<rocket::Error>),
    #[error("Logging error: {0}")]
    LoggingError(#[from] log::SetLoggerError),
    #[error("Argon2 Error: {0}")]
    Argon2Error(#[from] argon2::Error),
    #[error("Error verifying hashed password")]
    PasswordHashError,

    #[error("Unauthenticated user")]
    UnauthenticatedUser,
    #[error("User does not have access rights")]
    ForbiddenAccess,
    #[error("{0} Not found")]
    NotFound(String),
    #[error("Unknown route")]
    UnknownRoute,
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    InvalidResult(String),
    #[error("Internal error")]
    InternalError,
    #[error("User conflict")]
    UserConflict,
    #[error("Too many requests")]
    TooManyRequests,

    #[error("Configuration Error")]
    ConfigurationError,
    #[error("Application Configuration Error")]
    AppConfigurationError,
    #[error("Database URL not configured")]
    DatabaseNotConfigured,
    #[error("Config file not found")]
    ConfigFileNotFound,
    #[error("Empty DB Url")]
    EmptyDBUrl,
    #[error("{0}")]
    Config(#[from] config::ConfigError),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown Error")]
    Unknown,
}

impl Error {
    /// Converts an instance of the ErrorResponse enum to an equivalent HTTP status.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the ErrorResponse enum instance.
    ///
    /// # Returns
    ///
    /// The equivalent HTTP status for the given ErrorResponse.
    pub fn to_status(&self) -> Status {
        match *self {
            Self::UnauthenticatedUser => Status::Unauthorized,
            Self::ForbiddenAccess => Status::Forbidden,
            Self::BadRequest(_) | Self::InvalidResult(_) | Self::UserConflict => Status::BadRequest,
            Self::NotFound(_) | Self::UnknownRoute => Status::NotFound,
            Self::TooManyRequests => Status::TooManyRequests,
            _ => Status::InternalServerError,
        }
    }
}

impl From<rocket::serde::json::Error<'_>> for Error {
    /// Converts a `rocket::serde::json::Error` into an `Error` enum.
    ///
    /// # Arguments
    ///
    /// * `e` - The json error that needs to be converted.
    ///
    /// # Returns
    ///
    /// The converted `Error` enum.
    fn from(e: rocket::serde::json::Error<'_>) -> Self {
        Error::FormatError(format!("{:?}", e))
    }
}

impl Serialize for Error {
    /// Serializes the Error struct.
    ///
    /// # Arguments
    ///
    /// * `serializer` - The serializer used for serialization.
    ///
    /// # Returns
    ///
    /// The Result with the serialized value or an error if serialization fails.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 2)?;
        state.serialize_field("error", &self.to_string())?;
        state.serialize_field("code", &self.to_status().code)?;

        state.end()
    }
}

impl<'r> Responder<'r, 'static> for Error {
    /// Generates a response based on the specified request.
    ///
    /// # Arguments
    ///
    /// - `self`: The object on which this method is called.
    /// - `request`: The request to respond to.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `Response` if the generation was successful, or an error if it failed.
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        let status = self.to_status();

        response::Response::build_from(Json(self).respond_to(request)?)
            .status(status)
            .ok()
    }
}
