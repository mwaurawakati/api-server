/*use chrono::Utc;
use futures::stream::StreamExt;
use lambda_http::{Body, Request, Response, Error as LambdaError};
use multipart::server::Multipart;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tokio_util::io::ReaderStream;

pub async fn upload_file_endpoint(
    storage_path: &str,
    request: Request,
) -> Result<Response<Body>, LambdaError> {
    let boundary = match request.headers().get("content-type") {
        Some(ct) => {
            let ct_str = ct.to_str().unwrap_or("");
            let parts: Vec<&str> = ct_str.split("boundary=").collect();
            if parts.len() == 2 {
                Some(parts[1].to_string())
            } else {
                None
            }
        }
        None => None,
    };

    if boundary.is_none() {
        return Ok(Response::builder()
            .status(400)
            .body("Missing boundary".into())
            .expect("Failed to create response"));
    }

    let mut multipart = Multipart::with_body(request.into_body(), boundary.unwrap());

    while let Some(Ok(mut field)) = multipart.next().await {
        let content_disposition = field.headers.name.clone();
        let filename = match content_disposition {
            Some(name) => name.to_string(),
            None => {
                return Ok(Response::builder()
                    .status(400)
                    .body("Missing filename".into())
                    .expect("Failed to create response"));
            }
        };

        let now = Utc::now();
        let file_path = format!(
            "{}/{}/{:02}/{:02}/{}",
            storage_path,
            now.year(),
            now.month(),
            now.day(),
            filename
        );

        let mut file = File::create(&file_path).expect("Failed to create file");

        while let Some(chunk) = field.data().await {
            file.write_all(&chunk.expect("Failed to read chunk"))
                .expect("Failed to write chunk");
        }
    }

    Ok(Response::builder()
        .status(200)
        .body("File uploaded successfully".into())
        .expect("Failed to create response"))
}

pub async fn download_file_endpoint(
    storage_path: &str,
    file_path: &str,
) -> Result<Response<Body>, LambdaError> {
    let mut full_path = PathBuf::from(storage_path);
    full_path.push(file_path);

    if !full_path.exists() {
        return Ok(Response::builder()
            .status(404)
            .body("File not found".into())
            .expect("Failed to create response"));
    }

    let file = File::open(full_path).expect("Failed to open file");
    let stream = ReaderStream::new(file);
    let body = Body::wrap_stream(stream);

    Ok(Response::builder()
        .status(200)
        .body(body)
        .expect("Failed to create response"))
}
*/