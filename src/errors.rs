use std::error::Error as StdError;

use backtrace::Backtrace;
use futures::sync::mpsc::SendError;
use tokio::sync::mpsc::error::UnboundedRecvError;

use crate::proto::plugin::Packet;

pub trait ErrorExt: StdError {
    fn reason(&self) -> Option<&(dyn ErrorExt + 'static)> {
        None
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        None
    }
    fn kind(&self) -> ErrorKind;
    fn as_std_error(&self) -> &(dyn StdError + 'static);
}

#[derive(Clone, Debug, Display, From)]
pub enum ErrorKind {
    #[display(fmt = "io error")]
    Io,

    #[display(fmt = "mpsc error")]
    Mpsc,

    #[display(fmt = "encoding error")]
    Encoding,

    #[display(fmt = "decoding error")]
    Decoding,
}

#[derive(Debug, Display)]
#[display(fmt = "error: {}, cause: {:?}", kind, cause)]
pub struct Error {
    kind: ErrorKind,
    backtrace: Option<Backtrace>,
    cause: Option<Box<dyn ErrorExt + Send + Sync + 'static>>,
}

impl StdError for Error {}

impl ErrorExt for Error {
    fn kind(&self) -> ErrorKind {
        self.kind.clone()
    }

    fn as_std_error(&self) -> &(dyn StdError + 'static) {
        self
    }
}

impl Error {
    /// With kind
    pub fn with_kind(kind: ErrorKind) -> Self {
        let backtrace = Some(Backtrace::new());
        Error {
            kind,
            backtrace,
            cause: None,
        }
    }

    /// Chains an error with a cause
    pub fn with_cause<E>(kind: ErrorKind, cause: E) -> Self
    where
        E: ErrorExt + Send + Sync + 'static,
    {
        let backtrace = Some(match cause.backtrace() {
            Some(backtrace) => backtrace.clone(),
            None => Backtrace::new(),
        });
        let cause: Option<Box<(dyn ErrorExt + Sync + Send + 'static)>> = Some(Box::new(cause));
        Error {
            kind,
            backtrace,
            cause,
        }
    }
}

macro_rules! impl_error_type {
    ($ty:path, $kind:ident) => {
        impl From<$ty> for Error {
            fn from(err: $ty) -> Self {
                Self::with_cause(ErrorKind::$kind, err)
            }
        }

        impl ErrorExt for $ty {
            fn kind(&self) -> ErrorKind {
                ErrorKind::$kind
            }

            fn as_std_error(&self) -> &(dyn StdError + 'static) {
                self
            }
        }
    };
}

impl_error_type!(::std::io::Error, Io);
impl_error_type!(::prost::EncodeError, Encoding);
impl_error_type!(::prost::DecodeError, Decoding);

// other impls

impl From<UnboundedRecvError> for Error {
    fn from(err: UnboundedRecvError) -> Self {
        Self::with_kind(ErrorKind::Mpsc)
    }
}

impl From<SendError<Packet>> for Error {
    fn from(err: SendError<Packet>) -> Self {
        Self::with_kind(ErrorKind::Mpsc)
    }
}
