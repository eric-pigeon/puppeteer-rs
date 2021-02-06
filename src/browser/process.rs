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
        // TODO cleanup
        println!("{:?}", program.as_ref());
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
        for arg in args.clone() {
            print!("{:?}", arg.as_ref())
        }
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
    // usePipe?: boolean;
    // timeout: number;
    // slowMo: number;
    // preferredRevision: string;

    pub async fn setup_connection(&mut self, options: ConnectionOptions) -> Connection {
        let transport = if options.use_pipe {
            //   // stdio was assigned during start(), and the 'pipe' option there adds the
            //   // 4th and 5th items to stdio array
            //   const { 3: pipeWrite, 4: pipeRead } = this.proc.stdio;
            //   const transport = new PipeTransport(
            //     pipeWrite as NodeJS.WritableStream,
            //     pipeRead as NodeJS.ReadableStream
            //   );
            //   this.connection = new Connection('', transport, slowMo);
            unimplemented!();
        } else {
            let ws_endpoint = self
                .wait_for_ws_endpoint()
                .await
                .expect("failed to launch chrome process");
            WebSocketTransport::new(ws_endpoint).await
            //   const transport = await WebSocketTransport.create(browserWSEndpoint);
            //   this.connection = new Connection(browserWSEndpoint, transport, slowMo);
        };

        Connection::new(Box::new(transport))
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
        // return new Promise((resolve, reject) => {
        //   const rl = readline.createInterface({ input: browserProcess.stderr });
        //   let stderr = '';
        //   const listeners = [
        //     helper.addEventListener(rl, 'line', onLine),
        //     helper.addEventListener(rl, 'close', () => onClose()),
        //     helper.addEventListener(browserProcess, 'exit', () => onClose()),
        //     helper.addEventListener(browserProcess, 'error', (error) =>
        //       onClose(error)
        //     ),
        //   ];
        //   const timeoutId = timeout ? setTimeout(onTimeout, timeout) : 0;

        //   /**
        //    * @param {!Error=} error
        //    */
        //   function onClose(error?: Error): void {
        //     cleanup();
        //     reject(
        //       new Error(
        //         [
        //           'Failed to launch the browser process!' +
        //             (error ? ' ' + error.message : ''),
        //           stderr,
        //           '',
        //           'TROUBLESHOOTING: https://github.com/puppeteer/puppeteer/blob/main/docs/troubleshooting.md',
        //           '',
        //         ].join('\n')
        //       )
        //     );
        //   }

        //   function onTimeout(): void {
        //     cleanup();
        //     reject(
        //       new TimeoutError(
        //         `Timed out after ${timeout} ms while trying to connect to the browser! Only Chrome at revision r${preferredRevision} is guaranteed to work.`
        //       )
        //     );
        //   }

        //   function onLine(line: string): void {
        //     stderr += line + '\n';
        //     const match = line.match(/^DevTools listening on (ws:\/\/.*)$/);
        //     if (!match) return;
        //     cleanup();
        //     resolve(match[1]);
        //   }

        //   function cleanup(): void {
        //     if (timeoutId) clearTimeout(timeoutId);
        //     helper.removeEventListeners(listeners);
        //   }
        // });
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        println!("dropping child");
        self.inner.kill().expect("failed to kill chrome browser");
        self.inner.wait().expect("failed to wait on chrome browser");
    }
}
