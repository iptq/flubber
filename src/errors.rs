use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    IO(::std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::IO(err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl StdError for Error {}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Self {
        Error::IO(err)
    }
}
