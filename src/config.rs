use dirs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConnectionConfig {
    #[serde(rename = "unix")]
    UnixSocket { path: PathBuf },

    #[serde(rename = "tcp")]
    Tcp { addr: String, port: u16 },
    // TODO: fifo?
}

impl Default for ConnectionConfig {
    fn default() -> ConnectionConfig {
        let home = dirs::home_dir().unwrap();
        let path = home.join(".flubber.socket");
        ConnectionConfig::UnixSocket { path }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    client_connection: ConnectionConfig,
}
