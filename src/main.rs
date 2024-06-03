use api_server::server::init_server;
use rocket::error::Error;
use std::process::exit;

#[cfg(not(target_os = "windows"))]
use jemallocator::Jemalloc;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[rocket::main]
async fn main() -> Result<(), Error> {
    // start the server
    match init_server().await {
        Ok(server) => server.launch().await.map(|_| ()),
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    }
}
