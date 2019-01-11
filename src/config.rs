use std::net::SocketAddr;
use std::path::PathBuf;

use dirs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConnectionConfig {
    // TODO: add #[cfg(unix)] on all unix socket related variants in every enum
    #[serde(rename = "unix")]
    Unix { path: PathBuf },

    #[serde(rename = "tcp")]
    Tcp { addr: SocketAddr },

    // TODO: fifo?
}

impl Default for ConnectionConfig {
    fn default() -> ConnectionConfig {
        let home = dirs::home_dir().unwrap();
        let path = home.join(".flubber.socket");
        ConnectionConfig::Unix { path }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// Password to authenticate with (must not be empty)
    // TODO: check that it's not empty
    pub client_password: String,

    /// Connection information about the client-facing side.
    pub client_connection: ConnectionConfig,
}
