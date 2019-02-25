use std::error::Error as StdError;

use backtrace::Backtrace;

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
    #[display(fmt = "cbor error")]
    Cbor,

    #[display(fmt = "io error")]
    Io,

    #[display(fmt = "encoding error")]
    EncodingError,
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
    /// Chains an error with a cause
    pub fn with_cause<E>(kind: ErrorKind, cause: E) -> Self
    where
        E: ErrorExt + Send + Sync + 'static,
    {
        let backtrace = Some(match cause.backtrace() {
            Some(backtrace) => backtrace.clone(),
            None => Backtrace::new(),
        });
        let cause: Option<Box<dyn ErrorExt + Send + Sync + 'static>> = Some(Box::new(cause));
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
            fn from(error: $ty) -> Self {
                Self::with_cause(ErrorKind::$kind, error)
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
impl_error_type!(::serde_cbor::error::Error, Cbor);
