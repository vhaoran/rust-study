#[allow(dead_code)]
pub fn hello() {
    println!(" hello of test::a:hello");
}


#[test]
fn hello_test_1() {
    hello();
}