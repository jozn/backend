
pub struct UserParam {}

#[derive(Debug)]
pub enum GenErr {
    ReadingPbParam,
    NoRpcMatch,
    ProtocolBuffer(quick_protobuf::Error),
    HttpClientErr(reqwest::Error),
    ProstDecode(::prost::DecodeError),
    ProstEncode(::prost::EncodeError),
}

impl From<quick_protobuf::Error> for GenErr {
    fn from(e: quick_protobuf::Error) -> Self {
        GenErr::ProtocolBuffer(e)
    }
}

impl From<reqwest::Error> for GenErr {
    fn from(e: reqwest::Error) -> Self {
        GenErr::HttpClientErr(e)
    }
}

impl From<prost::DecodeError> for GenErr {
    fn from(e: ::prost::DecodeError) -> Self {
        GenErr::ProstDecode(e)
    }
}

impl From<prost::EncodeError> for GenErr {
    fn from(e: ::prost::EncodeError) -> Self {
        GenErr::ProstEncode(e)
    }
}
