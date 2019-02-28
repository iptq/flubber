use flubber::{
    proto::{plugin::Packet, PluginCodec},
    Error, ErrorKind,
};
use futures::{future, sync::mpsc, Future, Stream};
use irc::client::prelude::{Client, ClientExt, Command, Config, IrcClient};
use lazy_static::lazy_static;
use tokio::{
    io::{stdin, stdout},
    runtime::Runtime,
};
use tokio_codec::FramedWrite;

fn irc_future(to_flubber: mpsc::UnboundedSender<Packet>) -> impl Future<Item = (), Error = ()> {
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
    client
        .stream()
        .for_each(move |message| {
            use flubber::proto::plugin::{packet::Kind, PacketId, PluginNewMessage};
            if let Command::PRIVMSG(target, contents) = message.command {
                let new_message = PluginNewMessage {
                    timestamp: 0,
                    author: message.prefix.unwrap(),
                    contents: contents,
                };
                let kind = Kind::PluginNewMessage(new_message);
                sequence = sequence + 1;
                let packet = Packet {
                    id: PacketId {
                        origin: 1,
                        sequence,
                    },
                    kind: Some(kind),
                };
                to_flubber.send(packet);
            }
            future::ok(())
        })
        .map(|_| ())
        .map_err(|_| ())
}

fn stdin_future() -> impl Future<Item = (), Error = ()> {
    let stdin = stdin();
    future::ok::<_, ()>(())
}

fn stdout_future(rx: mpsc::UnboundedReceiver<Packet>) -> impl Future<Item = (), Error = ()> {
    let codec = PluginCodec::default();
    let stdout = stdout();
    // TODO: what the fuck?
    rx.map_err(|_| Error::with_kind(ErrorKind::Io))
        .forward(FramedWrite::new(stdout, codec))
        .map(|_| ())
        .map_err(|_| ())
}

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let (tx, rx) = mpsc::unbounded();
    runtime.spawn(stdout_future(rx));
    runtime.spawn(irc_future(tx));
    runtime.spawn(stdin_future());
    runtime.shutdown_on_idle().wait().unwrap();
}
