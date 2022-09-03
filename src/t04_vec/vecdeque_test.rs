use std::collections::VecDeque;

#[test]
fn vd_1() {
    //---------------------
    let mut l: VecDeque<i32> = VecDeque::with_capacity(2);
    println!("-----------capcity {}-----------", l.capacity());

    l.reserve(2);
    println!("-----------capcity {}-----------", l.capacity());

    l.push_back(1);
    l.push_front(2);
    l.push_back(3);
    println!("-----------{:?}-----------", l);
    println!("-----------capcity {}-----------", l.capacity());
}

#[test]
fn fib_1() {
    fn fib(u: u64) -> u64 {
        match u {
            0 => 0,
            1 => 1,
            _ => fib(n - 1) + fib(n - 2),
        }
    }

    for i in 10..20_u64 {
        println!("----------fib({i}) = {}-----------", fib(i));
    }
}
