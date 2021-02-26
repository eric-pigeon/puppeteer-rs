#[cfg(test)]
mod browser {
    use puppeteer::{Browser, LaunchOptions};

    #[cfg(test)]
    mod browser_contexts {
        use puppeteer::{Browser, LaunchOptions};

        #[tokio::test]
        async fn it_returns_all_contexts() {
            let _browser = Browser::launch(LaunchOptions::default())
                .await
                .expect("failed to launch browser");

            // TOOD
            assert_eq!(2 + 2, 4);
        }
    }

    #[tokio::test]
    async fn it_returns_browser_version() {
        let browser = Browser::launch(LaunchOptions::default())
            .await
            .expect("failed to launch browser");

        let version = browser.version().await;
        assert!(version.contains("Headless"));
    }

    #[tokio::test]
    async fn it_returns_user_agent() {
        let browser = Browser::launch(LaunchOptions::default())
            .await
            .expect("failed to launch browser");

        let user_agent = browser.user_agent().await;
        assert!(user_agent.contains("WebKit"));
    }
}
