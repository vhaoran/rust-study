//use std::error::Error;
use std::num::ParseIntError;

#[test]
fn err_test() {
    let a = match err_1() {
        Ok(v) => v,
        Err(_e) => -1000,
    };

    println!("----------------------");
    println!("{}", a);
}

fn err_1() -> Result<i32, ParseIntError> {
    let a = call()?;
    return Ok(a);

    fn call() -> Result<i32, ParseIntError> {
        "35aab".parse::<i32>()
    }
}