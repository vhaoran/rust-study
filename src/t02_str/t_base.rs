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
