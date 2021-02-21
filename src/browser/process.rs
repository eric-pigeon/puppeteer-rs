use super::connection::{Connection, ConnectionOptions, WebSocketTransport};
use regex::Regex;
use std::ffi::OsStr;
use std::io::{self, BufRead};
use std::process;
use std::process::Stdio;
use tempfile::TempDir;

pub struct Command {
    inner: process::Command,
    tmp_dir: TempDir,
}

pub struct Child {
    inner: process::Child,
    _tmp_dir: TempDir,
}

impl Command {
    pub fn new<S: AsRef<OsStr>>(program: S, tmp_dir: TempDir) -> Command {
        Command {
            inner: process::Command::new(program),
            tmp_dir: tmp_dir,
        }
    }

    pub fn args<I, S>(mut self, args: I) -> Command
    where
        I: IntoIterator<Item = S> + Clone,
        S: AsRef<OsStr>,
    {
        self.inner.args(args);
        self
    }

    // TODO handle interrupts
    pub fn spawn(mut self) -> io::Result<Child> {
        let child = self
            .inner
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        Ok(Child {
            inner: child,
            _tmp_dir: self.tmp_dir,
        })
    }
}

impl Child {
    pub async fn setup_connection(&mut self, options: ConnectionOptions) -> Connection {
        let (transport, read_stream) = if options.use_pipe {
            unimplemented!();
        } else {
            let ws_endpoint = self
                .wait_for_ws_endpoint()
                .await
                .expect("failed to launch chrome process");
            WebSocketTransport::new(ws_endpoint).await
        };

        Connection::new(Box::new(transport), read_stream)
    }

    // TODO does this need mut self?
    async fn wait_for_ws_endpoint(&mut self) -> Result<String, &'static str> {
        let re = Regex::new("^DevTools listening on (ws://.*)\n$").unwrap();

        let mut buffer = String::new();
        let mut reader = io::BufReader::new(self.inner.stderr.take().unwrap());
        while let Ok(_num_bytes) = reader.read_line(&mut buffer) {
            if let Some(captures) = re.captures(&buffer) {
                return Ok(captures.get(1).unwrap().as_str().to_string());
            }
            buffer.clear();
        }
        return Err("failed to get websocket endpoint");
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        self.inner.kill().expect("failed to kill chrome browser");
        self.inner.wait().expect("failed to wait on chrome browser");
    }
}
