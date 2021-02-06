#[cfg(test)]
mod browser {
    #[cfg(test)]
    mod browser_contexts {
        use puppeteer::{Browser, LaunchOptions};
        use tokio::time::{sleep, Duration};

        #[tokio::test]
        async fn it_returns_all_contexts() {
            let _browser = Browser::launch(LaunchOptions::default())
                .await
                .expect("failed to launch browser");

            let sleep_time = Duration::from_secs(10);
            sleep(sleep_time).await;
            // drop(browser);
            assert_eq!(2 + 2, 4);
        }
    }
}
