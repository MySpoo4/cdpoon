use std::{fmt, result};

#[derive(Debug)]
pub enum Error {
    Error { msg: String },
    RequestError { url: String, msg: String },
    DeserializeError { msg: String },
    ConnectionError { msg: String },
    WriteError { msg: String },
    ReadError { msg: String },
    NoConnectionError,
    NoMessage,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Error { msg } => write!(f, "Error message: '{}'", msg),
            Error::RequestError { url, msg } => {
                write!(f, "Failed to get from url: '{}'. Message: '{}'", url, msg)
            }
            Error::DeserializeError { msg } => {
                write!(f, "Failed to deserialize. Message: '{}'", msg)
            }
            Error::ConnectionError { msg } => write!(f, "Connection error message: '{}'", msg),
            Error::WriteError { msg } => write!(f, "Failed to write to socket. Message: '{}'", msg),
            Error::ReadError { msg } => write!(f, "Failed to read from socket. Message: '{}'", msg),
            Error::NoConnectionError => write!(f, "A connection has yet to been created"),
            Error::NoMessage => write!(f, "No message"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
