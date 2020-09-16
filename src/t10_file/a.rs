use std::fs;
// use std::env;


#[test]
fn test_read_file() {
    // let args = std::env::Args().collect();
    // println!("{:?}", args);

    let text = fs::read_to_string("/Users/whr/t.txt").unwrap();
    println!("{}", text);
}