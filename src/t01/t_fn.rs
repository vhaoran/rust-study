use std::borrow::Borrow;


#[test]
fn test_fn_1() {
    let mut a: i32 = 33;
    let mut b: i32 = 44;
    fn_1(&mut a, &mut b);
    println!("a: {} b: {}", a, b);
}

fn fn_1(a: &mut i32, b: &mut i32) {
    let t = *a;
    *a = *b;
    *b = t
}