use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

use futures::{
    sync::oneshot::{Receiver as OneshotReceiver, Sender as OneshotSender},
    Future, Poll,
};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_process::{Child, ChildStdin, ChildStdout, CommandExt};
use tower_service::Service;

use crate::proto::plugin::PacketId;
use crate::proto::Packet;
use crate::Error;

struct PluginInner {
    pub child: Child,
    pub stdin: ChildStdin,
    pub stdout: ChildStdout,
    pub pending_messages: HashMap<(), OneshotReceiver<()>>,
    pub sequence: i32,
}

#[derive(Clone)]
pub struct Plugin(Arc<Mutex<PluginInner>>);

impl Plugin {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let mut child = Command::new(path.as_ref())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn_async()?;
        let stdin = child.stdin().take().unwrap();
        let stdout = child.stdout().take().unwrap();
        let pending_messages = HashMap::new();
        Ok(Plugin(Arc::new(Mutex::new(PluginInner {
            child,
            stdin,
            stdout,
            pending_messages,
            sequence: 0,
        }))))
    }
}

impl Read for Plugin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let mut inner = self.0.lock().unwrap();
        inner.stdout.read(buf)
    }
}

impl AsyncRead for Plugin {}

impl Write for Plugin {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let mut inner = self.0.lock().unwrap();
        inner.stdin.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        let mut inner = self.0.lock().unwrap();
        inner.stdin.flush()
    }
}

impl AsyncWrite for Plugin {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        let mut inner = self.0.lock().unwrap();
        inner.stdin.shutdown()
    }
}
