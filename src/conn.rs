use futures::{Poll, Stream};
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};

use crate::errors::Error;

pub enum Connection {
    Unix(UnixListener),
    Tcp(TcpListener),
}

impl Connection {
    pub fn incoming(self) -> Incoming {
        match self {
            Connection::Unix(listener) => Incoming::Unix(listener.incoming()),
            Connection::Tcp(listener) => Incoming::Tcp(listener.incoming()),
        }
    }
}

pub enum Incoming {
    Unix(::tokio::net::unix::Incoming),
    Tcp(::tokio::net::tcp::Incoming),
}

impl Stream for Incoming {
    type Item = ConnStream;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self {
            Incoming::Unix(incoming) => incoming
                .poll()
                .map(|asyn| asyn.map(|opt| opt.map(|item| ConnStream::Unix(item))))
                .map_err(|err| err.into()),
            Incoming::Tcp(incoming) => incoming
                .poll()
                .map(|asyn| asyn.map(|opt| opt.map(|item| ConnStream::Tcp(item))))
                .map_err(|err| err.into()),
        }
    }
}

pub enum ConnStream {
    Unix(UnixStream),
    Tcp(TcpStream),
}
