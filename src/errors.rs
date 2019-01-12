use crate::proto::ClientMessage;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(::std::io::Error),
    Cbor(::serde_cbor::error::Error),
    ClientSend(::futures::sync::mpsc::SendError<ClientMessage>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::Io(err) => write!(f, "io error: {}", err),
            Error::Cbor(err) => write!(f, "cbor error: {}", err),
            Error::ClientSend(err) => write!(f, "client send error: {}", err),
        }
    }
}

impl StdError for Error {}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<::serde_cbor::error::Error> for Error {
    fn from(err: ::serde_cbor::error::Error) -> Self {
        Error::Cbor(err)
    }
}

impl From<::futures::sync::mpsc::SendError<ClientMessage>> for Error {
    fn from(err: ::futures::sync::mpsc::SendError<ClientMessage>) -> Self {
        Error::ClientSend(err)
    }
}
