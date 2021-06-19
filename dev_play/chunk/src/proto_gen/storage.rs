#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBucketRequest {
    #[prost(uint32, tag = "12")]
    pub bucket_id: u32,
    /// image video avatar music voice image_thumb document
    #[prost(string, tag = "3")]
    pub intent: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBucketResponse {
    /// if operation is ok
    #[prost(uint32, tag = "12")]
    pub bucket_id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileRequest {
    #[prost(fixed64, tag = "1")]
    pub file_id: u64,
    #[prost(fixed64, tag = "11")]
    pub ref_id: u64,
    ///?
    #[prost(uint32, tag = "12")]
    pub bucket_id: u32,
    #[prost(uint32, tag = "2")]
    pub secret: u32,
    /// FileType - this is enum from global.proto
    #[prost(uint32, tag = "3")]
    pub file_type: u32,
    #[prost(bytes = "vec", tag = "10")]
    pub blob_data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFileRequest {
    #[prost(fixed64, tag = "1")]
    pub file_id: u64,
    #[prost(fixed64, tag = "11")]
    pub ref_id: u64,
    #[prost(uint32, tag = "12")]
    pub bucket_id: u32,
    #[prost(uint32, tag = "2")]
    pub secret: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFileResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub ok: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
    #[prost(uint32, tag = "1")]
    pub id: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
    #[prost(uint32, tag = "1")]
    pub id: u32,
}
#[doc = r" Generated client implementations."]
pub mod client_to_chunk_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct ClientToChunkClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ClientToChunkClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ClientToChunkClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = "  rpc  (Request) returns (Response);"]
        pub async fn create_bucket(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBucketRequest>,
        ) -> Result<tonic::Response<super::CreateBucketResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/storage.ClientToChunk/CreateBucket");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn upload_file(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadFileRequest>,
        ) -> Result<tonic::Response<super::UploadFileResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/storage.ClientToChunk/UploadFile");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_file(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFileRequest>,
        ) -> Result<tonic::Response<super::RemoveFileResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/storage.ClientToChunk/RemoveFile");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<super::PingRequest>,
        ) -> Result<tonic::Response<super::PingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/storage.ClientToChunk/Ping");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ClientToChunkClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ClientToChunkClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ClientToChunkClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod client_to_chunk_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ClientToChunkServer."]
    #[async_trait]
    pub trait ClientToChunk: Send + Sync + 'static {
        #[doc = "  rpc  (Request) returns (Response);"]
        async fn create_bucket(
            &self,
            request: tonic::Request<super::CreateBucketRequest>,
        ) -> Result<tonic::Response<super::CreateBucketResponse>, tonic::Status>;
        async fn upload_file(
            &self,
            request: tonic::Request<super::UploadFileRequest>,
        ) -> Result<tonic::Response<super::UploadFileResponse>, tonic::Status>;
        async fn remove_file(
            &self,
            request: tonic::Request<super::RemoveFileRequest>,
        ) -> Result<tonic::Response<super::RemoveFileResponse>, tonic::Status>;
        async fn ping(
            &self,
            request: tonic::Request<super::PingRequest>,
        ) -> Result<tonic::Response<super::PingResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ClientToChunkServer<T: ClientToChunk> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ClientToChunk> ClientToChunkServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ClientToChunkServer<T>
    where
        T: ClientToChunk,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/storage.ClientToChunk/CreateBucket" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBucketSvc<T: ClientToChunk>(pub Arc<T>);
                    impl<T: ClientToChunk> tonic::server::UnaryService<super::CreateBucketRequest>
                        for CreateBucketSvc<T>
                    {
                        type Response = super::CreateBucketResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBucketRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_bucket(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateBucketSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/storage.ClientToChunk/UploadFile" => {
                    #[allow(non_camel_case_types)]
                    struct UploadFileSvc<T: ClientToChunk>(pub Arc<T>);
                    impl<T: ClientToChunk> tonic::server::UnaryService<super::UploadFileRequest> for UploadFileSvc<T> {
                        type Response = super::UploadFileResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UploadFileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).upload_file(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UploadFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/storage.ClientToChunk/RemoveFile" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFileSvc<T: ClientToChunk>(pub Arc<T>);
                    impl<T: ClientToChunk> tonic::server::UnaryService<super::RemoveFileRequest> for RemoveFileSvc<T> {
                        type Response = super::RemoveFileResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveFileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_file(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RemoveFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/storage.ClientToChunk/Ping" => {
                    #[allow(non_camel_case_types)]
                    struct PingSvc<T: ClientToChunk>(pub Arc<T>);
                    impl<T: ClientToChunk> tonic::server::UnaryService<super::PingRequest> for PingSvc<T> {
                        type Response = super::PingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).ping(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ClientToChunk> Clone for ClientToChunkServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ClientToChunk> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ClientToChunk> tonic::transport::NamedService for ClientToChunkServer<T> {
        const NAME: &'static str = "storage.ClientToChunk";
    }
}
