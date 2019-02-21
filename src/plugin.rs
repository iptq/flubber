use std::io::{self, Read, Write};
use std::path::Path;
use std::process::Command;

use futures::{Poll, Stream};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_process::{Child, ChildStdin, ChildStdout, CommandExt};

use crate::Error;

pub struct Plugin {
    managed: Child,
}

impl Plugin {
    pub fn spawn(path: impl AsRef<Path>) -> Result<Plugin, Error> {
        let process = Command::new(path.as_ref()).spawn_async()?;
        Ok(Plugin { managed: process })
    }
}

impl Read for Plugin {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let mut stdout = self.managed.stdout().as_mut();
        stdout.unwrap().read(buf)
    }
}

impl AsyncRead for Plugin {}

impl Write for Plugin {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let mut stdin = self.managed.stdin().as_mut();
        stdin.unwrap().write(buf)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        let mut stdin = self.managed.stdin().as_mut();
        stdin.unwrap().flush()
    }
}

impl AsyncWrite for Plugin {
    fn shutdown(&mut self) -> Poll<(), io::Error> {
        let mut stdin = self.managed.stdin().as_mut();
        stdin.unwrap().shutdown()
    }
}
