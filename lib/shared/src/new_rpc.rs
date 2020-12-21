use crate::pb;
use crate::pb::{EchoParam, EchoResponse};
use crate::{errors::GenErr, UserParam};
use async_trait::async_trait;

use http::Version;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error as HyperError, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

#[async_trait]
trait RPC_Shared {
    async fn Echo(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
}

#[async_trait]
trait RPC_Chat {
    async fn ChatSendMsg(up: &UserParam, param: pb::EchoParam) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
}

#[async_trait]
trait RPC_Chat2_Trait {
    async fn ChatSendMsg(
        &self,
        up: &UserParam,
        param: pb::EchoParam,
    ) -> Result<pb::EchoResponse, GenErr> {
        Ok(pb::EchoResponse::default())
    }
}

struct Rpc {
    RPC_Chat2: Box<dyn RPC_Chat2_Trait>,
    RPC_Chat3: Option<Box<dyn RPC_Chat2_Trait>>,
}

impl Rpc {
    pub fn new(RPC_Chat2: Box<dyn RPC_Chat2_Trait>) -> Self {
        Rpc {
            RPC_Chat2: RPC_Chat2,
            RPC_Chat3: None,
        }
    }
}

struct Ut {
    s: String,
}

impl Ut {
    pub fn serve(data: Vec<u8>) {}
}

#[async_trait]
impl RPC_Chat2_Trait for Ut {
    async fn ChatSendMsg(&self, up: &UserParam, param: EchoParam) -> Result<EchoResponse, GenErr> {
        unimplemented!()
    }
}

/////////////////////Http

pub type FHttpResponse = (u16, Vec<u8>);
pub struct FHttpRequest {
    method: http::Method,
    path: String,
    body: Vec<u8>,
}

trait ServeHttpRequest {
    // fn serve_http_old(req: &http::Request<Vec<u8>>) -> FHttpResponse;
}

pub struct SampleService {
    pub port: u16,
}

impl FIMicroService for SampleService {
    fn port(&self) -> u16 {
        4040
    }

    fn serve_http(&self, req: FHttpRequest) -> (u16, Vec<u8>) {
        (200, b"hi there".to_vec())
    }
}

#[async_trait]
pub trait FIMicroService {
    fn port(&self) -> u16;
    fn serve_http(&self, req: FHttpRequest) -> FHttpResponse;

    async fn server_http_requests(&self) {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port()));

        println!("There will some sort.");

        let make_svc = make_service_fn(move |_| async {
            Ok::<_, HyperError>(service_fn(move |_req| async {
                let m = 32;
                // let v = self.port;
                let res = Response::new(Body::from("Hello World"));
                Ok::<_, HyperError>(res)
            }))
        });

        let server = Server::bind(&addr).serve(make_svc);
        // println!("Server is running on port {}", self.port);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}

/*impl FMicroService {
    pub async fn server_http_requests(&self) {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        // let make_svc =
        //     make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(server_http)) });
        /*        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(
                |req: Request<Body>| -> Result<Response<Body>, Infallible> {
                    Ok(Response::builder().status(200).body("sdf".into()).unwrap())
                },
            ))
        });*/

        /*        let service = service_fn(|req: Request<Body>| async move {
            if req.version() == Version::HTTP_11 {
                Ok(Response::new(Body::from("Hello World")))
            } else {
                // Note: it's usually better to return a Response
                // with an appropriate StatusCode instead of an Err.
                Err("not HTTP/1.1, abort connection")
            }
        });*/

        let make_svc = make_service_fn(move |_| async {
            Ok::<_, HyperError>(service_fn(move |_req| async {
                let m = 32;
                // let v = self.port;
                Ok::<_, HyperError>(Response::new(Body::from("Hello World")))
            }))
        });

        let server = Server::bind(&addr).serve(make_svc);
        // println!("Server is running on port {}", self.port);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}*/

async fn server_http(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder().status(200).body("sdf".into()).unwrap())
}
