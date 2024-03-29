#[allow(unused_imports)]
#[allow(dead_code)]
use std::borrow::Borrow;
use std::path::{Path, PathBuf};

#[test]
fn file_t_2() {
    let dir = std::env::current_dir().unwrap();
    println!("------------{}-------------", dir.display());

    let p = Path::new(dir.to_str().unwrap());

    let p = p.join(PathBuf::from("aa.txt"));
    //
    println!("------after join filename------{}-------------", p.display());
}