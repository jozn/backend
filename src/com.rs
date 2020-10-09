
// pub struct GenErr {}
pub struct UserParam {}

#[derive(Debug)]
pub enum GenErr {
    ReadingPbParam,
    NoRpcMatch,
    ProtocolBuffer(quick_protobuf::Error),
    HttpClientErr(reqwest::Error),
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
