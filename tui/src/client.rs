use crate::Opt;
use flubber::{conn::{ConnFuture, ConnStream},ClientCodec};
use futures::Future;
use tokio::{net::{TcpStream, UnixStream}, io::AsyncRead};

pub fn run(args: Opt) {
    // establish a connection
    let conn = match (&args.connection.unix, &args.connection.tcp) {
        (Some(_), Some(_)) => panic!("Only one of --unix or --tcp should be used."),
        (Some(path), None) => ConnFuture::Unix(UnixStream::connect(path)),
        (None, Some(addr)) => ConnFuture::Tcp(TcpStream::connect(addr)),
        (None, None) => panic!("No connection method specified. Use either --unix or --tcp"),
    };

    tokio::run(conn.map(|socket| {
        let (stream, sink) = socket.split();
    }).map_err(|err| {
        eprintln!("Error: {}", err);
        ()
    }));
}
