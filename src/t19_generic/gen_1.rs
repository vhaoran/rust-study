use std::fmt::Display;

#[allow(dead_code)]
fn add<T>(a: T, b: T) -> T
    where T: Display + Ord + Clone + Copy + std::ops::Add<Output=T>
//     where T: Display + PartialOrd + Clone + Ord
{
    a + b
}


#[test]
fn gen_01() {
    let x: u8 = 3;

    let a = add(3, 4);
    println!("----gen_1.rs---{}-----", a);
    println!("----gen_1.rs---{}-----", x);
}
