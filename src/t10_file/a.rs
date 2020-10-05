// use std::env;


#[test]
fn test_read_file() {
    // let args = std::env::Args().collect();
    // println!("{:?}", args);
    use std::fs;

    let text = fs::read_to_string("/Users/whr/t.txt").unwrap();
    println!("{}", text);
}

/*
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
*/
//------------------------------------------------
/*
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
*/


