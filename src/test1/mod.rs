pub mod a;
pub mod b;
pub mod map_test;


pub fn hello() {
    println!(" hello of test::a:hello");
}

#[test]
fn hello_3_test() {
    hello();
}