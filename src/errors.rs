use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(::std::io::Error),
    Cbor(::serde_cbor::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::Io(err) => write!(f, "IO Error: {}", err),
            Error::Cbor(err) => write!(f, "CBOR Error: {}", err),
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
