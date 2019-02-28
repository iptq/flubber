// TODO: unhardcode this

use flubber::Daemon;
use futures::Future;
use tokio::runtime::Runtime;

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let mut daemon = Daemon::new();
    daemon
        .add_plugin("/home/michael/Projects/flubber/target/debug/irc_plugin")
        .unwrap();
    runtime.spawn(daemon.start().map_err(|_| ()));
    runtime.shutdown_on_idle().wait().unwrap();
}
