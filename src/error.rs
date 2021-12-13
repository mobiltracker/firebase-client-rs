use hyper::{http, StatusCode};

#[derive(Debug)]
pub enum FirebaseClientError {
    InvalidUriError(http::uri::InvalidUri),
    SerializeNotificationError(serde_json::Error),
    BuildRequestError(hyper::http::Error),
    HttpRequestError {
        status_code: StatusCode,
        body: String,
    },
    ClientError(hyper::Error),
    ReadBodyError(hyper::Error),
}

impl From<http::uri::InvalidUri> for FirebaseClientError {
    fn from(err: http::uri::InvalidUri) -> Self {
        FirebaseClientError::InvalidUriError(err)
    }
}

impl From<hyper::http::Error> for FirebaseClientError {
    fn from(err: hyper::http::Error) -> Self {
        FirebaseClientError::BuildRequestError(err)
    }
}

impl From<hyper::Error> for FirebaseClientError {
    fn from(err: hyper::Error) -> Self {
        FirebaseClientError::ClientError(err)
    }
}

impl From<serde_json::Error> for FirebaseClientError {
    fn from(err: serde_json::Error) -> Self {
        FirebaseClientError::SerializeNotificationError(err)
    }
}
