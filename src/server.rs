use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Response, StatusCode};
use hyper::header::{CONTENT_TYPE, HeaderValue};
use hyper::service::{make_service_fn, service_fn};
use include_dir::{Dir, include_dir};
use mime_guess;

static ASSETS: Dir = include_dir!("static/");

pub async fn start() -> hyper::Result<()> {
    let make_svc = make_service_fn(move |_conn| {
        async {
            Ok::<_, Infallible>(service_fn(move |req|
                async move {
                    handle(req).await
                }
            ))
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let server = hyper::Server::bind(&addr).serve(make_svc);

    println!("Server is running on http://{}", addr);

    server.await
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


async fn handle(_req: hyper::Request<Body>) -> Result<Response<Body>, Infallible> {
    let original_path = _req.uri().path().to_string();
    let path = static_path(original_path);
    // let path = if _req.uri().path() == "/" {
    //     "/index.html"
    // } else {
    //     _req.uri().path()
    // };
    // let path = &path[1..];
    eprintln!("path {}", path);
    let file_path = path.as_str();
    let asset = ASSETS.get_file(file_path);

    match asset {
        Some(file) => {
            let mime = mime_guess::from_path(file_path).first_or_octet_stream();
            Ok(Response::builder()
                .header(CONTENT_TYPE, HeaderValue::from_str(mime.as_ref()).unwrap())
                .body(Body::from(file.contents()))
                .unwrap())
        }
        None => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("404 - Not Found"))
                .unwrap())
        }
    }
}