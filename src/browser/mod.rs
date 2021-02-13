use process::{Child, Command};
use std::path::PathBuf;
use tempfile::tempdir;

use browser_fetcher::BrowserFetcher;
use connection::{Connection, ConnectionOptions};
pub use context::BrowserContext;
pub use launch_options::{BrowserOptions, ChromeArgOptions, LaunchOptions};

use crate::protocol::target;

mod browser_fetcher;
mod connection;
mod context;
mod launch_options;
mod process;

const DEFAULT_ARGS: &'static [&'static str] = &[
    "--disable-background-networking",
    "--enable-features=NetworkService,NetworkServiceInProcess",
    "--disable-background-timer-throttling",
    "--disable-backgrounding-occluded-windows",
    "--disable-breakpad",
    "--disable-client-side-phishing-detection",
    "--disable-component-extensions-with-background-pages",
    "--disable-default-apps",
    "--disable-dev-shm-usage",
    "--disable-extensions",
    "--disable-features=Translate",
    "--disable-hang-monitor",
    "--disable-ipc-flooding-protection",
    "--disable-popup-blocking",
    "--disable-prompt-on-repost",
    "--disable-renderer-backgrounding",
    "--disable-sync",
    "--force-color-profile=srgb",
    "--metrics-recording-only",
    "--no-first-run",
    "--enable-automation",
    "--password-store=basic",
    "--use-mock-keychain",
    // TODO(sadym): remove "--enable-blink-features=IdleDetection"
    // once IdleDetection is turned on by default.
    "--enable-blink-features=IdleDetection",
];

pub struct Browser {
    _child: Child,
    _connection: Connection,
}

impl Browser {
    pub async fn launch(launch_options: LaunchOptions) -> Result<Browser, std::io::Error> {
        let tmp_dir = tempdir().expect("failed to create tmp dir");
        let mut args = DEFAULT_ARGS.to_vec();

        if launch_options.chrome_arg_options.headless {
            args.extend(&["--headless", "--hide-scrollbars", "--mute-audio"])
        };
        if launch_options.pipe {
            args.extend(&["--remote-debugging-pipe"])
        } else {
            args.extend(&["--remote-debugging-port=0"])
        }
        let tmp_dir_path = tmp_dir
            .path()
            .to_str()
            .expect("invalid temp directory path");
        let user_data_dir = &format!("--user-data-dir={}", tmp_dir_path);
        args.push(user_data_dir);
        let executable_path = match resolve_executable() {
            Ok(path) => path,
            Err(err) => panic!(err),
        };
        // TODO cleanup
        // println!(
        //     "executable_path {}",
        //     executable_path.to_str().expect("path")
        // );

        let mut child = Command::new(executable_path, tmp_dir).args(args).spawn()?;
        let connection = child
            .setup_connection(ConnectionOptions {
                use_pipe: launch_options.pipe,
                timeout: launch_options.timeout,
                slow_mo: launch_options.browser_options.slow_mo,
            })
            .await;

        let browser = Browser {
            _child: child,
            _connection: connection,
        };

        connection.send(target::SetDiscoverTargets { discover: true });

        Ok(browser)
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        println!("Dropping browser");
        //     let _ = self.loop_shutdown_tx.send(());
        //     self.transport.shutdown();
    }
}

fn resolve_executable() -> Result<PathBuf, &'static str> {
    let fetcher = BrowserFetcher::new();
    fetcher.fetch()
}
