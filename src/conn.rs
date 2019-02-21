use std::io::{self, Read, Write};

use futures::{Future, Poll, Stream};
use tokio::net::{TcpListener, TcpStream, UnixListener, UnixStream};
use tokio_io::{AsyncRead, AsyncWrite};

use crate::errors::Error;

pub enum Listener {
    Unix(UnixListener),
    Tcp(TcpListener),
}

impl Listener {
    pub fn incoming(self) -> Incoming {
        match self {
            Listener::Unix(listener) => Incoming::Unix(listener.incoming()),
            Listener::Tcp(listener) => Incoming::Tcp(listener.incoming()),
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

pub enum ConnFuture {
    Unix(::tokio::net::unix::ConnectFuture),
    Tcp(::tokio::net::tcp::ConnectFuture),
}

impl Future for ConnFuture {
    type Item = ConnStream;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self {
            ConnFuture::Unix(future) => future
                .poll()
                .map(|asyn| asyn.map(|stream| ConnStream::Unix(stream)))
                .map_err(|err| err.into()),
            ConnFuture::Tcp(future) => future
                .poll()
                .map(|asyn| asyn.map(|stream| ConnStream::Tcp(stream)))
                .map_err(|err| err.into()),
        }
    }
}

pub enum ConnStream {
    Unix(UnixStream),
    Tcp(TcpStream),
}

impl Read for ConnStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        match self {
            ConnStream::Unix(stream) => stream.read(buf),
            ConnStream::Tcp(stream) => stream.read(buf),
        }
    }
}

impl Write for ConnStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        match self {
            ConnStream::Unix(stream) => stream.write(buf),
            ConnStream::Tcp(stream) => stream.write(buf),
        }
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        match self {
            ConnStream::Unix(stream) => stream.flush(),
            ConnStream::Tcp(stream) => stream.flush(),
        }
    }
}

impl AsyncRead for ConnStream {}

impl AsyncWrite for ConnStream {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        match self {
            ConnStream::Unix(stream) => stream.shutdown(),
            ConnStream::Tcp(stream) => stream.shutdown(),
        }
    }
}
