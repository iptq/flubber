use flubber::{
    proto::{Codec, Packet},
    Error, ErrorKind,
};
use futures::{future, sync::mpsc, Future, Stream};
use irc::client::prelude::{Client, ClientExt, Command, Config, IrcClient};
use lazy_static::lazy_static;
use tokio::{
    io::{stdin, stdout},
    runtime::Runtime,
};
use tokio_codec::{FramedRead, FramedWrite};

fn irc_future(
    to_flubber: mpsc::UnboundedSender<Packet>,
    from_flubber: mpsc::UnboundedReceiver<Packet>,
) -> impl Future<Item = (), Error = ()> {
    let config = Config {
        server: Some("acm.umn.edu".to_owned()),
        nickname: Some("flubber".to_owned()),
        port: Some(6669),
        use_ssl: Some(true),
        channels: Some(vec!["#flubber".to_owned()]),
        ..Default::default()
    };

    let client = IrcClient::from_config(config).unwrap();
    let mut sequence = 0;
    client.identify().unwrap();
    let a = client
        .stream()
        .for_each(move |message| {
            use flubber::proto::{packet::Kind, plugin::PluginIncomingMessage, PacketId};
            use std::time::{SystemTime, UNIX_EPOCH};
            if let Command::PRIVMSG(target, contents) = message.command {
                let new_message = PluginIncomingMessage {
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("time went backwards")
                        .as_millis() as u64,
                    author: message.prefix.unwrap(),
                    contents: contents,
                };
                let kind = Kind::PluginIncomingMessage(new_message);
                sequence = sequence + 1;
                let packet = Packet {
                    id: Some(PacketId {
                        origin: 1,
                        sequence,
                    }),
                    kind: Some(kind),
                };
                // TODO: don't unwrap
                to_flubber.unbounded_send(packet).unwrap();
            }
            future::ok(())
        })
        .map_err(|err| {
            eprintln!("error: {}", err);
        });
    let b = from_flubber.for_each(|packet| {
        use flubber::proto::{packet::Kind, plugin::PluginIncomingMessage, PacketId};
        match packet.kind {
            _ => (),
        }
        future::ok(())
    });
    a.join(b).map(|_| ())
}

fn stdin_future(tx: mpsc::UnboundedSender<Packet>) -> impl Future<Item = (), Error = ()> {
    let codec = Codec::<Packet>::new();
    let stdin = stdin();
    let framed_read = FramedRead::new(stdin, codec);
    framed_read.forward(tx).map(|_| ()).map_err(|_| ())
}

fn stdout_future(rx: mpsc::UnboundedReceiver<Packet>) -> impl Future<Item = (), Error = ()> {
    let codec = Codec::<Packet>::new();
    let stdout = stdout();
    // TODO: what the fuck?
    rx.map_err(|_| Error::with_kind(ErrorKind::Io))
        .forward(FramedWrite::new(stdout, codec))
        .map(|_| ())
        .map_err(|err| {
            eprintln!("error: {}", err);
        })
}

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let (to_flubber_tx, to_flubber_rx) = mpsc::unbounded();
    let (from_flubber_tx, from_flubber_rx) = mpsc::unbounded();
    runtime.spawn(stdout_future(to_flubber_rx));
    runtime.spawn(irc_future(to_flubber_tx, from_flubber_rx));
    runtime.spawn(stdin_future(from_flubber_tx));
    runtime.shutdown_on_idle().wait().unwrap();
}
