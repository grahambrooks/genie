use axum::{http, Json, Router, routing::get};
use axum::body::Body;
use axum::extract::Request;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::put;
use include_dir::{Dir, include_dir};
use serde_json::Value;

static ASSETS: Dir = include_dir!("static/");

pub async fn start() -> std::io::Result<()> {
    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Genie listening at http://localhost:3000");
    axum::serve(listener, app).await
}

fn app() -> Router {
    Router::new()
        .route("/api/chats", get(get_chats))
        .route("/api/chats", put(put_chat))
        .fallback(fallback)
}

async fn get_chats(_request: Request) -> Json<Value> {
    Json(serde_json::json!({ "chats": [] }))
}

async fn put_chat(_request: Request) -> impl IntoResponse {
    let data = serde_json::json!({ "chat": {} });
    (StatusCode::CREATED, Json(data))
}

async fn fallback(uri: Uri) -> Response {
    let filename = static_path(uri.path().to_string());
    let content_type = mime_guess::from_path(filename.clone()).first_or_octet_stream();
    let file = ASSETS.get_file(filename).unwrap();
    // Create and return a response with an appropriate content type
    // and the file contents as the body.
    Response::builder()
        .status(StatusCode::OK)
        .header(http::header::CONTENT_TYPE, content_type.as_ref())
        .body(Body::from(file.contents()))
        .unwrap()
}

fn static_path(original_path: String) -> String {
    if original_path == "/" {
        return "index.html".to_string();
    }

    if original_path.starts_with("/") {
        return original_path[1..].to_string();
    }
    original_path.to_string()
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body
        ,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    // for `collect`
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use super::*;

    #[test]
    fn test_static_path() {
        assert_eq!(static_path("/".to_string()), "index.html".to_string());
        assert_eq!(static_path("/index.html".to_string()), "index.html".to_string());
        assert_eq!(static_path("/static/index.html".to_string()), "static/index.html".to_string());
    }

    #[tokio::test]
    async fn homepage() {
        let app = app();

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        // convert body to string
        let b = String::from_utf8(body.to_vec()).unwrap();
        assert!(b.contains("Genie"));
    }

    #[tokio::test]
    async fn get_chats() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/api/chats")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({"chats":[]}));
    }

    #[tokio::test]
    async fn put_chat() {
        let app = app();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri("/api/chats")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_vec(&json!([1, 2, 3, 4])).unwrap(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({"chat":{}}));
    }
}