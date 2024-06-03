use crate::{controllers, db::SqliteBackend, error::Error, secure::guards::cors::Cors, Result};
use clap::Parser;
use log::{info, LevelFilter};
use rocket::{data::Limits, Build, Config, Rocket};
use rocket_okapi::{
    openapi_get_routes,
    rapidoc::{
        make_rapidoc, ApiConfig, ApiKeyLocation, GeneralConfig, HideShowConfig, LayoutConfig,
        RapiDocConfig, RenderStyle, Theme, UiConfig,
    },
    settings::UrlObject,
};
use simple_logger::SimpleLogger;
use std::path::Path;

/// Server & App Configurations
pub mod config;
use self::config::Settings;

/// Catchers like 500, 501, 404, etc
mod catchers;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct CliOpts {
    /// loads the server configurations
    #[clap(short = 'c', long)]
    config: String,
}

/// Parse the settings from the command line arguments
fn parse_settings_from_cli() -> Result<Settings> {
    // parse the cli options
    let cli_opts = CliOpts::parse();
    let cfg_file = &cli_opts.config;

    if cfg_file.is_empty() {
        // No config file, so start
        // with default settings
        Ok(Settings::default())
    } else {
        // Config file passed in cli, check
        // to see if config file exists
        if Path::new(cfg_file).exists() {
            // load settings from the config file or return error
            // if error in loading the given config file
            Settings::from_file(cfg_file)
        } else {
            // config file does not exist, quit app
            Err(Error::ConfigFileNotFound)
        }
    }
}

/// Initialise the Rocket Server app
pub async fn init_server() -> Result<Rocket<Build>> {
    let settings = parse_settings_from_cli()?;
    SimpleLogger::new()
        .with_level(to_level_filter(&settings.server.log_level))
        .with_colors(true)
        .init()
        .unwrap();

    info!(
        "Starting Server with logging level: {:?}",
        settings.server.log_level
    );
    info!("Server CORS enabled: {:?}", settings.server.allow_cors);
    let db_url = if let Some(a) = settings.app {
        a.db_path
    } else {
        return Err(Error::AppConfigurationError);
    };
    if db_url.is_empty() {
        return Err(Error::DatabaseNotConfigured);
    }

    let server_settings = settings.server;

    // Uses the secret key to encrypt the Password. So if the
    // secret key is lost/changed, the password cannot be decrypted.
    let salt = server_settings.secret_key.to_owned();

    let limits = Limits::new()
        .limit("forms", server_settings.forms_limit.into())
        .limit("json", server_settings.json_limit.into());

    let rocket_cfg = Config::figment()
        .merge(("address", server_settings.host.to_string()))
        .merge(("port", server_settings.port as u16))
        .merge(("limits", limits))
        .merge(("secret_key", (server_settings.secret_key.as_str())))
        .merge(("keep_alive", server_settings.keep_alive as u32));

    let db_backend = SqliteBackend::new_connection(&db_url).await?;
    db_backend.check_and_create_table().await?;

    // Configure the Rocket server with configured settings
    let app = rocket::custom(rocket_cfg);

    // Catchers
    let app = app.register(
        "/",
        rocket::catchers![
            catchers::bad_request,
            catchers::forbidden,
            catchers::not_authorized,
            catchers::not_found,
            catchers::unprocessed_entity,
            catchers::internal_server_error,
            catchers::too_many_requests,
        ],
    );

    // Add the routes with openapi specs
    let app = app
        .mount(
            "/",
            openapi_get_routes![
                controllers::users::create_user_endpoint,
                controllers::users::update_user_endpoint,
                controllers::users::delete_user_endpoint,
                controllers::users::list_all_users_endpoint,
                controllers::users::get_user_by_id_endpoint,
                controllers::users::get_user_by_api_key_endpoint,
            ],
        )
        .mount(
            "/docs/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("Api Specs", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    allow_search: true,
                    allow_authentication: true,
                    allow_try: true,
                    show_header: false,
                    ..Default::default()
                },
                ui: UiConfig {
                    theme: Theme::Light,
                    ..Default::default()
                },
                layout: LayoutConfig {
                    render_style: RenderStyle::Read,
                    ..Default::default()
                },
                api: ApiConfig {
                    api_key_name: "x-api-key".to_string(),
                    api_key_location: Option::from(ApiKeyLocation::Header),
                    api_key_value: "x-api-key".to_string(),
                    ..Default::default()
                },
                ..Default::default()
            }),
        );

    // Add the routes without openapi specs
    // let app = app.mount(
    //     "/",
    //     routes![
    //         controllers::users::create_user_endpoint,
    //         controllers::users::update_user_endpoint,
    //         controllers::users::delete_user_endpoint,
    //         controllers::users::list_all_users_endpoint,
    //         controllers::users::get_user_by_id_endpoint,
    //         controllers::users::get_user_by_api_key_endpoint,
    //     ]
    // );

    let app = app
        // Add Db pool to the state
        .manage(db_backend)
        // Sending the salt key to the state
        .manage(salt);

    // Attach Cors if disabled
    let app = if server_settings.allow_cors {
        app.attach(Cors)
    } else {
        app
    };

    // Return the configured Rocket App
    Ok(app)
}

/// Convert LevelFilter from string
fn to_level_filter(level: &str) -> LevelFilter {
    match level.to_uppercase().as_str() {
        "NONE" | "NO" | "FALSE" | "OFF" => LevelFilter::Off,
        "TRACE" => LevelFilter::Trace,
        "DEBUG" => LevelFilter::Debug,
        "WARN" | "WARNING" => LevelFilter::Warn,
        "ERROR" => LevelFilter::Error,
        _ => LevelFilter::Info,
    }
}


use aws_config::BehaviorVersion;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Deserialize)]
struct Request {
    body: String,
}

#[derive(Debug, Serialize)]
struct Response {
    req_id: String,
    body: String,
}

impl std::fmt::Display for Response {
    /// Display the response struct as a JSON string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_as_json = serde_json::json!(self).to_string();
        write!(f, "{err_as_json}")
    }
}

impl std::error::Error for Response {}

#[tracing::instrument(skip(s3_client, event), fields(req_id = %event.context.request_id))]
pub async fn put_object(
    s3_client: &aws_sdk_s3::Client,
    bucket_name: &str,
    event: LambdaEvent<Request>,
) -> Result<Response, Error> {
    tracing::info!("handling a request");
    // Generate a filename based on when the request was received.
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|n| n.as_secs())
        .expect("SystemTime before UNIX EPOCH, clock might have gone backwards");

    let filename = format!("{timestamp}.txt");
    let response = s3_client
        .put_object()
        .bucket(bucket_name)
        .body(event.payload.body.as_bytes().to_owned().into())
        .key(&filename)
        .content_type("text/plain")
        .send()
        .await;

    match response {
        Ok(_) => {
            tracing::info!(
                filename = %filename,
                "data successfully stored in S3",
            );
            // Return `Response` (it will be serialized to JSON automatically by the runtime)
            Ok(Response {
                req_id: event.context.request_id,
                body: format!(
                    "the Lambda function has successfully stored your data in S3 with name '{filename}'"
                ),
            })
        }
        Err(err) => {
            // In case of failure, log a detailed error to CloudWatch.
            tracing::error!(
                err = %err,
                filename = %filename,
                "failed to upload data to S3"
            );
            Err(Box::new(Response {
                req_id: event.context.request_id,
                body: "The Lambda function encountered an error and your data was not saved"
                    .to_owned(),
            }))
        }
    }
}


