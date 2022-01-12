use headless_chrome::LaunchOptionsBuilder;
use headless_chrome::{protocol::page::ScreenshotFormat, Browser};

#[tokio::test]
async fn hl_1() -> Result<(), Box<dyn std::error::Error>> {
    println!("--------before---start-----------");
    let opt = LaunchOptionsBuilder::default().headless(true).build()?;
    let browser = Browser::new(opt)?;
    println!("-----------a-----------");

    let tab = browser.wait_for_initial_tab()?;

    /// Navigate to wikipedia
    tab.navigate_to("http://www.zgei.com/")?;
    println!("-----------after after navigate-----------");

    /// Wait for network/javascript/dom to make the search-box available
    /// and click it.
    // tab.wait_for_element("input#searchInput")?.click()?;
    tab.wait_for_element("#j-input")?;
    tab.type_str("似水流年")?;

    tab.wait_for_element("#j-submit")?;
    println!("---after show j-submit-----");

    tab.find_element("button#j-submit")?.click()?;
    println!("----after click <j-submit>-----");

    /// We should end up on the WebKit-page once navigated
    // let r = tab.wait_until_navigated()?;
    let r = tab.wait_for_element("#j-lrc")?;
    let s = r.get_attributes()?;

    println!("-------lrc:----{:#?}-----------", r);
    println!("-----------after query-----------");
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    Ok(())
}
