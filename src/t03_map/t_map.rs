#[test]
fn t03() {
    //
    map_demo();
}

use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
fn map_demo() {
    {
        let mut m = HashMap::new();
        m.insert(String::from("aa"), 10);
        m.insert(String::from("b"), 20);
        m.entry(String::from("c")).or_insert(30);

        for (k, v) in &m {
            println!("----------------------");
            println!(" key: {},value: {}", k, v);
            println!(" key: {},value: {}", k, v);
            println!(" key: {},value: {}", k, v);
        }
        println!("------map----------------");
        println!("{:?}", m);
    }
    {}
}

#[test]
fn map_n() {
    //---------------------
    let mut m: HashMap<String, i64> = HashMap::new();
    m.insert("aa".to_string(), 1);
    m.insert("aa".to_string(), 2);
    m.insert("bb".to_string(), 1);
    m.insert("bb".to_string(), 3);
    println!("----------------------");
    for (k, v) in m {
        println!(" {:?},{:?}", k, v);
    }
}

#[test]
fn hs_2() {
    //---------------------
    let mut a :HashSet<i32> = HashSet::new();
    //
    a.insert(1);
    a.insert(5);
    a.insert(7);
}

