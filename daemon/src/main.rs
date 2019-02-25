// TODO: unhardcode this

use flubber::Daemon;

fn main() {
    let mut daemon = Daemon::new();
    daemon.add_plugin("target/debug/irc_plugin").unwrap();
    daemon.start()
}
