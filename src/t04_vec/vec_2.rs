use std::ops::Add;
// use std::io::Read;

#[test]
fn ve_01() {
    let s = "abc中国";
    let l = s.as_bytes();
    //-------------------------------------
    let mut i = 0;
    for v in l.iter() {
        i = i.add(1);
        println!("----vec_2.rs---{}---{}--", v, i);
    }

    let s = std::str::from_utf8(l.iter().as_slice());
    match s {
        Ok(ss) => {
            println!("---{}------", ss);
            ()
        }
        _ => ()
    }
    // println!("----vec_2.rs---{}-----", s);
}