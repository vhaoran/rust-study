use std::option::Option;

#[test]
fn option_test() {
    let a: Option<i32> = opt_1();
    println!("----------------------");
    println!("{}", a.as_ref().unwrap());
    println!("{}", a.as_ref().unwrap());
    println!("{}", a.as_ref().unwrap());
}

#[allow(dead_code)]
fn opt_1() -> Option<i32> {
    Some(10)
}


//-----------------------------------------------------------------------------
#[test]
fn option_test2() {
    let a: Option<String> = opt_2();
    println!("----------------------");
    println!("{}", a.as_ref().unwrap());
    println!("{}", a.as_ref().unwrap());
    println!("{}", a.as_ref().unwrap());
}

#[allow(dead_code)]
fn opt_2() -> Option<String> {
    Some(String::from("abc"))
}

//-------------------------------------------------------------------
