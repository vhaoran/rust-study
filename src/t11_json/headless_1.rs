use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{protocol::page::ScreenshotFormat, Browser};

#[tokio::test]
async fn hl_1() -> Result<(), Box<dyn std::error::Error>> {
    println!("--------before---start-----------");
    let opt = LaunchOptionsBuilder::default().headless(true).build()?;
    let browser = Browser::new(opt)?;
    println!("-----------a-----------");

    let tab = browser.wait_for_initial_tab()?;
    // http://www.kuwo.cn/play_detail/150602055

    /// Navigate to wikipedia
    let url =
        "http://www.kuwo.cn/search/list?key=%E5%90%AC%E9%97%BB%E8%BF%9C%E6%96%B9%E6%9C%89%E4%BD%A0";
    let r = tab.navigate_to(url)?;
    println!("-----------after after navigate-----------");
    // println!("-----------{:#?}-----------",r);


    /// Wait for network/javascript/dom to make the search-box available
    /// and click it.
    // tab.wait_for_element("input#searchInput")?.click()?;
    // tab.wait_for_element("#j-input")?;


    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    Ok(())
}
