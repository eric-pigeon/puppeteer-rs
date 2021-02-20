use crate::viewport::Viewport;

pub struct LaunchOptions {
    pub executable_path: Option<String>,
    // ignore_default_args: bool,
    pub handle_sigint: bool,
    pub handle_sigterm: bool,
    pub handle_sighup: bool,
    pub timeout: u32,
    pub dump_io: bool,
    //env?: Record<string, string | undefined>;
    pub pipe: bool,

    pub browser_options: BrowserOptions,
    pub chrome_arg_options: ChromeArgOptions,
}

pub struct ChromeArgOptions {
    pub headless: bool,
    pub args: Vec<String>,
    pub user_data_dir: Option<String>,
    pub devtools: bool,
}

pub struct BrowserOptions {
    pub ignore_https_error: bool,
    pub default_viewport: Viewport,
    pub slow_mo: Option<i32>,
}

impl Default for ChromeArgOptions {
    fn default() -> ChromeArgOptions {
        ChromeArgOptions {
            // headless: false,
            headless: true,
            args: vec![],
            user_data_dir: None,
            devtools: false,
        }
    }
}

impl Default for LaunchOptions {
    fn default() -> Self {
        LaunchOptions {
            executable_path: None,
            handle_sigint: true,
            handle_sigterm: true,
            handle_sighup: true,
            timeout: 3000,
            dump_io: false,
            pipe: false,

            browser_options: Default::default(),
            chrome_arg_options: Default::default(),
        }
    }
}

impl Default for BrowserOptions {
    fn default() -> BrowserOptions {
        BrowserOptions {
            ignore_https_error: false,
            default_viewport: Viewport::default(),
            slow_mo: None,
        }
    }
}
