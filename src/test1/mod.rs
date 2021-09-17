pub mod a;
pub mod b;
pub mod map_test;
mod test_ajson;

#[allow(dead_code)]
pub fn hello() {
    println!(" hello of test::aa:hello");
}

#[test]
fn hello_3_test() {
    hello();
}
