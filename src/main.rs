use api_server::server::init_server;
use rocket::error::Error;
use std::process::exit;

#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

/*#[rocket::main]
async fn main() -> Result<(), Error> {
    // start the server
    match init_server().await {
        Ok(server) => server.launch().await.map(|_| ()),
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    }
}*/

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let bucket_name = "mwaurawakati"; /*std::env::var("BUCKET_NAME")
        .expect("A BUCKET_NAME must be set in this app's Lambda environment variables.");*/

    // Initialize the client here to be able to reuse it across
    // different invocations.
    //
    // No extra configuration is needed as long as your Lambda has
    // the necessary permissions attached to its role.
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_client = aws_sdk_s3::Client::new(&config);

    lambda_runtime::run(service_fn(|event: LambdaEvent<Request>| async {
        put_object(&s3_client, &bucket_name, event).await
    }))
    .await
}
