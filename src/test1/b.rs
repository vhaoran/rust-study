pub fn hello() {
    println!(" hello of test1::hello");
}

#[test]
fn hello_2_test() {
    hello();
}