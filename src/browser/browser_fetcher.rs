use std::path::PathBuf;

pub struct BrowserFetcher {}

impl BrowserFetcher {
    pub fn new() -> Self {
        Self {}
    }

    pub fn fetch(&self) -> Result<PathBuf, &'static str> {
        match std::env::consts::OS {
            "linux" => {
                unimplemented!("TODO")
            }
            "macos" => Ok(PathBuf::from(
                "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            )),
            "ios" => {
                unimplemented!("TODO")
            }
            "freebsd" => {
                unimplemented!("TODO")
            }
            "dragonfly" => {
                unimplemented!("TODO")
            }
            "netbsd" => {
                unimplemented!("TODO")
            }
            "openbsd" => {
                unimplemented!("TODO")
            }
            "solaris" => {
                unimplemented!("TODO")
            }
            "android" => {
                unimplemented!("TODO")
            }
            "windows" => {
                unimplemented!("TODO")
            }
            _ => Err("Unknown operating system"),
        }
    }
}
