use lambda_http::{Body, Request, Response};
use std::collections::HashMap;

async fn json_response(
    status_code: u16,
    message: &str,
    additional_fields: Option<HashMap<&str, String>>,
) -> Response<Body> {
    let mut response = HashMap::new();
    response.insert("status", status_code.to_string());
    response.insert("message", message.to_string());

    if let Some(fields) = additional_fields {
        for (key, value) in fields {
            response.insert(key, value);
        }
    }

    let body = Body::Text(serde_json::to_string(&response).unwrap());

    Response::builder()
        .status(status_code)
        .body(body)
        .expect("failed to render response")
}

pub async fn bad_request(req: &Request) -> Response<Body> {
    let uri = req.uri().to_string();
    let mut fields = HashMap::new();
    fields.insert("request_uri", uri);
    json_response(400, "request not understood", Some(fields)).await
}

pub async fn not_authorized(req: &Request) -> Response<Body> {
    let uri = req.uri().to_string();
    let mut fields = HashMap::new();
    fields.insert("request_uri", uri);
    json_response(401, "not authorized", Some(fields)).await
}

pub async fn forbidden(req: &Request) -> Response<Body> {
    let uri = req.uri().to_string();
    let mut fields = HashMap::new();
    fields.insert("request_uri", uri);
    json_response(403, "forbidden", Some(fields)).await
}

pub async fn not_found(req: &Request) -> Response<Body> {
    let uri = req.uri().to_string();
    let mut fields = HashMap::new();
    fields.insert("request_uri", uri);
    json_response(404, "not found", Some(fields)).await
}

pub async fn unprocessed_entity(_req: &Request) -> Response<Body> {
    json_response(422, "Check your input data", None).await
}

pub async fn too_many_requests(req: &Request) -> Response<Body> {
    let uri = req.uri().to_string();
    let mut fields = HashMap::new();
    fields.insert("request_uri", uri);
    json_response(429, "too many requests", Some(fields)).await
}

pub async fn internal_server_error(req: &Request) -> Response<Body> {
    let uri = req.uri().to_string();
    let mut fields = HashMap::new();
    fields.insert("request_uri", uri);
    json_response(500, "internal server error", Some(fields)).await
}
