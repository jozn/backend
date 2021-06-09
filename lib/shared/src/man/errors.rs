use crate::man::mysql_common::MyError;

#[derive(Debug)]
pub enum GenErr {
    ReadingPbParam,
    NoRpcMatch,
    HttpClientErr(reqwest::Error),
    ProstDecode(::prost::DecodeError),
    ProstEncode(::prost::EncodeError),
    MySQL(MyError),

    NoRpcRegistry,

    NotFound,
    WrongParam,

    UserSpaceErr,
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

impl From<MyError> for GenErr {
    fn from(err: MyError) -> Self {
        GenErr::MySQL(err)
    }
}
