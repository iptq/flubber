use crate::Opt;
use flubber::{conn::ConnFuture, ClientCodec, ClientMessage};
use futures::{
    future,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    Future, Sink, Stream,
};
use tokio::{
    io::AsyncRead,
    net::{TcpStream, UnixStream},
};
use tokio_codec::{FramedRead, FramedWrite};

pub fn run(
    args: Opt,
    from_ui: UnboundedReceiver<ClientMessage>,
    to_ui: UnboundedSender<ClientMessage>,
) {
    // establish a connection
    let conn = match (&args.connection.unix, &args.connection.tcp) {
        (Some(_), Some(_)) => panic!("Only one of --unix or --tcp should be used."),
        (Some(path), None) => ConnFuture::Unix(UnixStream::connect(path)),
        (None, Some(addr)) => ConnFuture::Tcp(TcpStream::connect(addr)),
        (None, None) => panic!("No connection method specified. Use either --unix or --tcp"),
    };

    tokio::run(
        conn.map(|socket| {
            let (stream, sink) = socket.split();

            let framed_read = FramedRead::new(stream, ClientCodec);
            let reader = framed_read
                .for_each(|message| future::ok(()))
                .map(|_| ());

            // write the auth message
            let framed_write = FramedWrite::new(sink, ClientCodec);
            let auth = ClientMessage::Auth { password: None };
            let writer = framed_write.send(auth).and_then(|stream| {
                from_ui.map_err(|_| unreachable!())
                    .fold(stream, |w, message| w.send(message))
                    .map(|_| ())
            });

            reader.select(writer).map_err(|_| ())
        })
        .map(|_| ())
        .map_err(|err| {
            eprintln!("Error: {}", err);
            ()
        }),
    );
}
