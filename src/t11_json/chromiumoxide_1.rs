use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use log::*;

#[tokio::test]
async fn xide_1() -> Result<(), Box<dyn std::error::Error>> {
    println!("-----------start-----------");
    pretty_env_logger::init();
    // log::set_level(LogLevel::Debug);
    debug!("----aaaa-----");
    println!("-----------aaa-----------");

    let (mut browser, mut handler) = Browser::launch(
        BrowserConfig::builder()
            // .with_head()
            // .no_sandbox()
            // .respect_https_errors()
            .build()?,
    )
    .await?;
    println!("----after lauch-----");

    let h = tokio::task::spawn(async move {
        loop {
            println!("----before-----");

            let _ = handler.next().await.unwrap();
            println!("----after-----");
        }
    });

    println!("-----------before page-----------");

    let url = "http://www.zgei.com";
    let page = browser.new_page(url).await?;
    println!("-----------....  0  ...  -----------");

    page.wait_for_navigation_response().await?;
    println!("-----------1-----------");

    // page.find_element("input#searchInput")
    page.find_element("input#j-input")
        .await?
        .type_str("\\u957f\\u57ce")
        .await?;
    println!("-----------2-----------");

    page.find_element("button#j-submit").await?.click().await?;
    println!("-----------3-----------");

    let _html = page.wait_for_navigation().await?.content().await?;
    println!("-----------4-----------");

    //wait

    h.await;
    Ok(())
}
