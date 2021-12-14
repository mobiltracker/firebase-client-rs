use std::{error, fmt};

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

impl fmt::Display for FirebaseClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FirebaseClientError::InvalidUriError(i) => write!(f, "InvalidUriError: {}", i),
            FirebaseClientError::SerializeNotificationError(err) => {
                write!(f, "SerializeNotificationError: {}", err)
            }
            FirebaseClientError::BuildRequestError(err) => write!(f, "BuildRequestError: {}", err),
            FirebaseClientError::HttpRequestError { status_code, body } => {
                write!(f, "HttpRequestError status:{} body:{}", status_code, body)
            }
            FirebaseClientError::ClientError(err) => write!(f, "ClientError: {}", err),
            FirebaseClientError::ReadBodyError(err) => write!(f, "ReadBodyError: {}", err),
        }
    }
}

impl error::Error for FirebaseClientError {}
