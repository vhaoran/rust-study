use std::ops::Index;

#[test]
fn test_str() {
    let a = "aaaa".to_string();
    let b = a.clone();
    let c = b.clone();

    println!("a: {} b: {}", a, b);
    println!("a: {} b: {}", a, b);
    println!("a: {} b: {}", a, b);
    println!("a: {} b: {}", a, b);
    println!("s: {} ", c);
    println!("s: {} ", c);
    println!("s: {} ", c);
    println!("s: {} ", c);
}

#[test]
fn str_2() {
    let s = "abc";
    let b = s.starts_with("a");
    println!("------------{}-------------", b);
    println!("------contains Of-b-----{}-------------",
             s.contains("a"));

    println!("------find Of-a---index--{}-------------",
             s.find("a").unwrap());
    println!("------find Of-ba--index---{}-------------",
             match s.find("ba") {
                 Some(v) => format!("{}", v),
                 _ => "not found".to_string(),
             });
}

#[test]
fn match_1() {
    let i = 0;
    let c = match i {
        >
    0 => 5,
    _ => 4,
}
    }