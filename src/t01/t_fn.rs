//use std::borrow::Borrow;


#[test]
fn test_fn_1() {
    let mut a: i32 = 33;
    let mut b: i32 = 44;
    fn_1(&mut a, &mut b);
    println!("aa: {} b: {}", a, b);

    fn_2(&mut a, &mut b);
    println!("-=-------------------------------");
    println!("aa: {} b: {}", a, b);
}

#[allow(dead_code)]
fn fn_1(a: &mut i32, b: &mut i32) {
    let t = *a;
    *a = *b;
    *b = t
}

#[allow(dead_code)]
fn fn_2(a: &mut i32, b: &mut i32) {
    //(*aa, *b) = (*b, *aa);
    let (aa, bb) = (*a, *b);
    *a = bb;
    *b = aa;
}

#[test]
fn fn_param() {
    type F = fn();

    fn a(f: F) {
        f();
        println!("------------call f()-------------");
    }

    fn b() {
        println!("------------call b-------------");
    }

    a(b);
    a(b);
    a(b);
}


