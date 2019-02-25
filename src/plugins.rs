use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use futures::{
    sync::oneshot::{Receiver as OneshotReceiver, Sender as OneshotSender},
    Poll,
};
use serde_cbor::Value;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_process::{Child, ChildStdin, ChildStdout, CommandExt};

use crate::message::Origin;

pub struct Plugin {
    child: Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
    pending_messages: HashMap<Origin, OneshotReceiver<Value>>,
}

impl Plugin {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let mut child = Command::new(path.as_ref())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn_async()?;
        let stdin = child.stdin().take().unwrap();
        let stdout = child.stdout().take().unwrap();
        let pending_messages = HashMap::new();
        Ok(Plugin {
            child,
            stdin,
            stdout,
            pending_messages,
        })
    }
}

impl Read for Plugin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.stdout.read(buf)
    }
}

impl AsyncRead for Plugin {}

impl Write for Plugin {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.stdin.write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.stdin.flush()
    }
}

impl AsyncWrite for Plugin {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        self.stdin.shutdown()
    }
}
