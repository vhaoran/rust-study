



use std::ops::Add;
#[test]
fn a_1_test() {
    //---------------------
    let s = "uudllr";
    let a = call(s);

    println!("----l_01.rs---{}--" ,a);
}

fn  call(s : &str)->bool{
    let mut u:i32 = 0;
    let mut l:i32 = 0;
    for c in s.chars(){
       match c{
           'u'=>u = u-1,
           'd'=>u = u+1,
           'l'=>l = l + 1,
           'r'=>l = l - 1,
           _=>(),
       }
    }
    u + l == 0
}