extern crate toml;

#[test]
fn t_toml() {
    #![deny(warnings)]
    use serde_derive::Deserialize;

    /// This is what we're going to decode into. Each field is optional, meaning
    /// that it doesn't have to be present in TOML.
    #[derive(Debug, Deserialize)]
    struct Config {
        ws_port: Option<u64>,
        ws_send_pwd: Option<String>,
        auth_url: Option<Vec<String>>,

        // ws_port = 8888
        // ws_send_pwd = "password"
        // auth_url = ["http://127.0.0.1:9110/InnerJwt"]
    }

    let toml_str = r#"
        ws_port = 8888
        ws_send_pwd = "password"
        auth_url = ["http://127.0.0.1:9110/InnerJwt"]
    "#;

    let c: Config = toml::from_str(toml_str).unwrap();
    println!("{:#?}", c);
    println!("-----------------------{}--", c.ws_port.unwrap());
    println!("-----------------------{}--", c.ws_send_pwd.unwrap());

    for i in c.auth_url.unwrap().iter() {
        println!("---------------------each:-{}---", i);
    }
}


#[test]
fn time_singe_1() {
    let count = 10;
    let i = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() % count;
    println!("------------{}-------------", i);
}