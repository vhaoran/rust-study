use std::collections::VecDeque;

#[test]
fn vd_1() {
    //---------------------
    let mut l :VecDeque<i32> = VecDeque::with_capacity(2);
    println!("-----------capcity {}-----------",l.capacity());

    l.reserve(2);
    println!("-----------capcity {}-----------",l.capacity());

    l.push_back(1);
    l.push_front(2);
    l.push_back(3);
    println!("-----------{:?}-----------",l);
    println!("-----------capcity {}-----------",l.capacity());

}