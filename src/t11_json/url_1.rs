extern crate url;
extern crate base64_url;
extern crate urlencoding;

#[allow(unused_imports)]
#[allow(dead_code)]
#[test]
fn url_1() {
    use url::{Url, Host};
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";
    let url = Url::parse(
        s
    );

    if let Ok(r) = url {
        println!("----url_1.rs---{}-----", r);
    }
    println!("-------------------------");
}

#[test]
fn url_2() {
    let url = "/ws?jwt=test%7C1";
    let r = base64_url::unescape(url);
    let s = r.to_string();
    println!("------------{}-------------", s);
}

#[test]
fn url_3() {
    let url = "/ws?jwt='test|1'";
    let mut s = String::new();
    let _r = base64_url::encode_to_string(url, &mut s);
    println!("------------{}-------------", s);
}

#[test]
fn url_decode_1() {
    use urlencoding::decode;
    let url = "/ws?jwt=test%7C1";
    let s = decode(url);
    println!(" before {}", url);
    println!(" after {}", s.unwrap());
    // 👾 Exterminate!
}

#[test]
fn aaa() {
    let t: Result<_, ()> = Ok("ping");
    match t {
        Ok("ping") => println!("---is ping 0-0----"),
        _ => {
            println!("-------------------------");
        }
    }
    println!("-------------------------");
}
