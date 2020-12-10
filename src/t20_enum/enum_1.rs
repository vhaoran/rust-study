#[test]
fn enum_t_01() {
    let r: Result<i32, ()> = Ok(3);

    match r {
        Ok(x) => println!(" r is {}", x),
        _ => println!(" r is none"),
    }
    //
}

#[test]
fn enum_t_02() {
    enum X<Y, N> {
        Yes(Y),
        No(N),
    }

    let r: X<i32, ()> = X::Yes(3);
    match r {
        X::Yes(x) => println!("good {}", x),
        _ => println!("none"),
    }
}