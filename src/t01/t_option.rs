use std::option::Option;
//use std::iter::Map;
use std::collections::hash_map::HashMap;


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

//-----------------------------------------------------------------------------
#[test]
fn option_test3() {
    opt_3();
}

#[allow(dead_code)]
fn opt_3() {
    let mut m = HashMap::new();
    m.insert("a".to_string(), "a_value".to_string());
    m.insert("b".to_string(), "b_value".to_string());

    let v = m.get("ba");
    //-------------------------------------------------------------------
    println!("----------------------");


    if v.is_some() {
        println!("v vluue is : {}", v.as_ref().unwrap());
    } else {
        println!("---------v is none-------------");
    }
    println!("----------------------");
}


//-------------------------------------------------------------------
#[test]
fn option_test4() {
    option_4();
}

fn option_4() {
    println!("----------------------");
    let a: Result<i64, i64> = Ok(4);

    println!("-------------------{}---", a.unwrap());
    let _r = match get() {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    };
}

fn get() -> Result<i64, i64> {
    Ok(18)
}
