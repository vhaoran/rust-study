extern crate ws;

#[allow(dead_code)]
pub fn hello() {
    println!(" hello of test::aa:hello");
}


#[test]
fn hello_test_1() {
    hello();
}

#[allow(unused_imports)]
#[allow(dead_code)]
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
