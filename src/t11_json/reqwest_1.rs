use reqwest;
use std::collections::HashMap;

#[tokio::test]
async fn req_1() -> Result<(), Box<dyn std::error::Error>> {
    // let url = "http://www.zgei.com";
    let url =
        "http://www.kuwo.cn/search/list?key=%E5%90%AC%E9%97%BB%E8%BF%9C%E6%96%B9%E6%9C%89%E4%BD%A0";
    // let resp = reqwest::Client::new()
    //     .post(url)
    //     .form(&[
    //         ("input", "在那遥远的地方"),
    //         ("filter", "name"),
    //         ("type", "netease"),
    //         ("page", "1"),
    //     ])
    //     .send()
    //     .await?;
    // .json::<HashMap<String, String>>()
    // .await?;

    let resp = reqwest::get(url).await?;

    let text = resp.text().await?;
    println!("{:#?}", text);
    Ok(())
}
