use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Result, Server, StatusCode};
use url::Url;
use urlparse::GetQuery;

use crate::types::*;

pub async fn listen_http(cfg :&Config) {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    // let addr = "127.0.0.1:9000".parse().unwrap();
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), cfg.port);

    let make_service =
        make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(process_http_request)) });

    let server = Server::bind(&socket_addr).serve(make_service);

    println!("Listening on http://{}", socket_addr);
    println!("Current directory {:?}", std::env::current_dir().unwrap());

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn process_http_request(req: Request<Body>) -> Result<Response<Body>> {
    // println!("{:?}", req);
    match (req.method(), req.uri().path()) {
        // (&Method::GET, "/") | (&Method::GET, "/index.html") => simple_file_send(INDEX).await,
        (&Method::GET, "/v1/get_file") => process_get_file(req).await,
        (&Method::GET, "/no_file.html") => {
            // Test what happens when file cannot be be found
            simple_file_send("this_file_should_not_exist.html").await
        }
        _ => { simple_file_send(req.uri().path().to_string().trim_start_matches("/")).await },
        // _ => Ok(not_found()),
    }
}

async fn process_get_file(req: Request<Body>) -> Result<Response<Body>> {
    use tokio_util::codec::{BytesCodec, FramedRead};

    let r = req.uri().query().unwrap_or("");

    let req: FileReqParam = serde_qs::from_str(r).unwrap();

    println!("{:} >> \n {:?}", r, req);

    let filename = &req.file;
    if let Ok(file) = tokio::fs::File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }

    Ok(not_found())
}

/// HTTP status code 404
fn not_found() -> Response<Body> {
    static NOTFOUND: &[u8] = b"Not Found";
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(NOTFOUND.into())
        .unwrap()
}

async fn simple_file_send(filename: &str) -> Result<Response<Body>> {
    use tokio_util::codec::{BytesCodec, FramedRead};
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.

    if let Ok(file) = tokio::fs::File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }

    Ok(not_found())
}
