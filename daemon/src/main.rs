// TODO: unhardcode this

use flubber::Daemon;
use futures::Future;

fn main() {
    let mut daemon = Daemon::new();
    daemon.add_plugin("target/debug/irc_plugin").unwrap();
    tokio::run(daemon.start().map_err(|_| ()));
}
