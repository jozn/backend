use crate::pb;
use crate::{errors::GenErr, UserParam};
use async_trait::async_trait;

use crate::rpc2;
use http::Version;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error as HyperError, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

pub type FHttpResponse = (u16, Vec<u8>);

#[derive(Debug)]
pub struct FHttpRequest {
    pub method: http::Method,
    pub path: String,
    pub body: bytes::Bytes,
}

trait ServeHttpRequest {
    // fn serve_http_old(req: &http::Request<Vec<u8>>) -> FHttpResponse;
}

pub struct SampleService {
    pub port: u16,
}

#[async_trait]
impl FIMicroService for SampleService {
    fn port(&self) -> u16 {
        4040
    }

    async fn serve_request(req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
        Ok((200, b"hi there".to_vec()))
    }
}

#[async_trait]
pub trait FIMicroService: Send + 'static {
    fn port(&self) -> u16;
    async fn serve_request(req: FHttpRequest) -> Result<FHttpResponse, GenErr>;
    async fn serve_request2(&self, req: FHttpRequest) -> Result<FHttpResponse, GenErr> {
        Ok((404, vec![]))
    }

    async fn listen_http_requests(&self) {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port()));
        println!("There will some sort.");

        // let celf = self;
        // NOT USED
        let serlf_arc = std::sync::Arc::new(self);
        let serlf_arc2 = serlf_arc.clone();

        let make_svc = make_service_fn(move |_| async {
            Ok::<_, HyperError>(service_fn(move |_req| async {
                let method = _req.method().clone();
                let path = _req.uri().path().to_string();
                let bo = _req.into_body();
                let bts = hyper::body::to_bytes(bo).await.unwrap();

                let req = FHttpRequest {
                    method: method,
                    path: path,
                    body: bts,
                };

                let response_res = Self::serve_request(req).await;
                match response_res {
                    Ok(rpc_res) => {
                        let res = Response::new(Body::from(rpc_res.1));
                        Ok::<_, HyperError>(res)
                    }
                    Err(err) => {
                        println!("Gateway Err: {:?}", err);
                        // Return empty body //todo: can send 5xx in here?
                        let res = Response::new(Body::from(""));
                        Ok::<_, HyperError>(res)
                    }
                }
            }))
        });

        let server = Server::bind(&addr).serve(make_svc);

        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    }
}
