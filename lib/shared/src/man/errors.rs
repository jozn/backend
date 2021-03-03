use crate::xc::CWError;

#[derive(Debug)]
pub enum GenErr {
    ReadingPbParam,
    NoRpcMatch,
    HttpClientErr(reqwest::Error),
    ProstDecode(::prost::DecodeError),
    ProstEncode(::prost::EncodeError),

    CassadraError,
    NoRpcRegistry,

    NotFound,
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

impl From<CWError> for GenErr {
    fn from(cerr: CWError) -> Self {
        GenErr::CassadraError
    }
}

/*impl
From<Option::None> for GenErr {
    fn from(_: Option::None) -> Self {
        GenErr::NotFound
    }
}
*/
/*impl From<std::option::NoneError> for GenErr {
    fn from(_: std::option::NoneError) -> Self {
        GenErr::NotFound
    }
}
*/
