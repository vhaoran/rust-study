extern crate serde_json;

use std::borrow::Borrow;


#[test]
fn test_01_json() {
    f_j01();
}


fn f_j01() {
    use serde_json::json;
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}

#[test]
fn test_j02() {
    use serde_json::{Result, Value, Value::String};
    fn untyped_example() -> Result<()> {
        // Some JSON input data as a &str. Maybe this comes from the user.
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data)?;

        // Access parts of the data by indexing with square brackets.
        println!("name: {}  phone: {}", v["name"], v["phones"][0]);


        match v["name"].borrow() {
            String(s) => println!("s = {}", s),
            _ => println!("no match strings")
        }

        println!("name: {}  phone: {}", v["name"], v["phones"][0]);
        Ok(())
    }

    let a = untyped_example();

    if let Ok(()) = a {
        println!(" ok of a");
        println!(" --------------");
    }
}