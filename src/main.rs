use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::body;
use serde::{Deserialize, Serialize};

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let s = format!("{:#?}", req);
    let h =  req.headers();
    let host = h.get(http::header::HOST);
    // let host = h.get(http::header::HOST);
    let uri = req.uri();
    // println!("uri >>> {:#?}", uri);
    println!("uri >>> {:#?}", uri.path());
    println!("uri >>> {:#?}", uri.query());
    println!("method >>> {:#?}", req.method());
    let p = uri.path();
    match p {
        "/echo" => Ok(Response::new(Body::from(uri.query().unwrap_or("[none]").to_string().clone()))),
        "/repeat" => Ok(Response::new(Body::from(s.repeat(100)))),
        "/rpc" => {
            // let bo = req.body().clone();
            let bo = req.into_body();
            let bts = body::to_bytes(bo).await.unwrap();
            let res_body = format!("body bytes:: {:#?}", bts);
            println!("{:?}", bts);
            Ok(Response::new(Body::from(res_body)))
            // Ok(Response::builder().status(404).body(Body::from("RPC not found.")).unwrap())
        },
        _ => Ok(Response::builder().status(404).body(Body::from("Not found.")).unwrap())
    }
    // Ok(Response::new(Body::from(s.repeat(100))))
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

struct Act {
    method: u32,
    data: String,
    act_id: u64,
}


// println!("uri >>> {:#?}", uri.port());
// println!("uri >>> {:#?}", uri.host());
// println!("uri >>> {:#?}", host);
// println!("> {:#?}",  h.get(http::header::ACCEPT_ENCODING));
// println!("> {:#?}",  h.get(http::header::ACCEPT_CHARSET));
// println!("> {:#?}",  h.get(http::header::FORWARDED));
// Ok(Response::new("Hello, World".into()))
// Body::empty();
// match uri.
