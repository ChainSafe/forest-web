use std::error::Error;

use thirtyfour::prelude::*;

pub static WEBPAGE_URL: &str = "http://127.0.0.1:8787";
pub static CHROME_DRIVER_URL: &str = "http://127.0.0.1:9515";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut cap = DesiredCapabilities::chrome();
    cap.set_headless()?;
    let chrome = WebDriver::new(CHROME_DRIVER_URL, cap).await?;
    chrome.goto(WEBPAGE_URL).await?;

    Ok(())
}
