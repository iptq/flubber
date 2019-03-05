// TODO: unhardcode this

use flubber::{Daemon, ErrorExt};
use futures::Future;
use tokio::runtime::Runtime;

fn main() {
    let mut runtime = Runtime::new().unwrap();
    let mut daemon = Daemon::new();
    daemon
        .add_plugin("/home/michael/Projects/flubber/target/debug/irc_plugin")
        .unwrap();

    runtime.spawn(daemon.run().map_err(|err| {
        eprintln!("daemon error: {}", err);
        eprintln!("{:?}", err.backtrace())
    }));

    runtime.shutdown_on_idle().wait().unwrap();
}
