extern crate ws;

#[allow(dead_code)]
pub fn hello() {
    println!(" hello of test::a:hello");
}


#[test]
fn hello_test_1() {
    hello();
}


use ws::{Builder, Sender, Settings};

#[test]
fn ws_test_1() {
    let url = r#"ws://0755yicai.com:8083/ws?jwt=test|1"#;
    Builder::new()
        .with_settings(Settings {
            max_connections: 1,
            ..Settings::default()
        })
        .build(|out: Sender| move |msg| out.send(msg))
        .unwrap()
        .listen(url)
        .unwrap();
}
