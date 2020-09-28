extern crate isahc;


#[test]
fn http_01() {
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



