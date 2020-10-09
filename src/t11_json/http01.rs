extern crate isahc;


#[allow(unused_imports)]
#[allow(dead_code)]
use std::time::Duration;


#[test]
#[allow(unused_imports)]
#[allow(dead_code)]

fn http_get_01() {
    #[allow(unused_imports)]
    #[allow(dead_code)]

    use isahc::prelude::*;

    fn x() -> Result<(), isahc::Error> {
        // Send a GET request and wait for the response headers.
        // Must be `mut` so we can read the response body.
        let mut response = isahc::get("http://example.org")?;

        // Print some basic info about the response to standard output.
        println!("Status: {}", response.status());
        println!("Headers: {:#?}", response.headers());

        // Read the response body as text into a string and print it.
        print!("{}", response.text()?);
        Ok(())
    }
    let _ = x();
}

#[test]
fn http_post_01() {
    #[allow(unused_imports)]
    #[allow(dead_code)]

    use isahc::prelude::*;

    fn x() -> Result<(), isahc::Error> {
        let url = "http://127.0.0.1:9110/InnerJwt";
        let jwt = "test/1".to_string();
        let body = format!(r#"{{
                "Jwt": "{}"
            }}"#, jwt);

        let mut response = Request::post(url)
            .header("Content-Type", "application/json")
            .timeout(Duration::from_secs(3))
            .body(body)?
            .send()?;

        // Print some basic info about the response to standard output.
        println!("Status: {}", response.status());
        println!("Headers: {:#?}", response.headers());

        // Read the response body as text into a string and print it.
        print!("text : {}", response.text().unwrap().clone());

        Ok(())
    }
    let _ = x();
}

#[test]
fn fmt_1() {
    let aaa = "aaa".to_string();
    let s = format!(r#"{{"uname":"{}"  }}"#, aaa);
    println!("----{}-------", s);
}

#[test]
fn fmt_2() {
    let jwt = "test/1";
    let body = format!(r#"{{
          "jwt": "{}"
        }}"#, jwt);
    println!("-------------------------");
    println!("------------{}-------------", body);
}


