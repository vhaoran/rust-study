use std::path::{Path, PathBuf};
use std::borrow::Borrow;

#[test]
fn file_t_2() {
    let dir = std::env::current_dir().unwrap();
    println!("------------{}-------------", dir.display());

    let p = Path::new(dir.to_str().unwrap());

    let p = p.join(PathBuf::from("a.txt"));
    //
    println!("------after join filename------{}-------------", p.display());
}