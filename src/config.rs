use std::net::SocketAddr;
use std::path::PathBuf;

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
        ConnectionConfig::Tcp {
            addr: "127.0.0.1:5060".parse().unwrap(),
        }
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
