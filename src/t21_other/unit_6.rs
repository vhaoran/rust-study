#[test]
fn u_01() {
    //---------------------
    struct Counter {
        value: u64, // <1>
    }

    impl Counter {
        fn new() -> Self { // <2> <3>
            Counter { value: 0 } // <4> <5>
        }

        fn incr(&mut self) { // <6>
            self.value += 1;
        }
    }

    fn x() {
        let mut counter = Counter::new();

        counter.incr();
        counter.incr();

        println!("{}", counter.value);
    }
    //-------------------------------------
    x();
}

#[test]
fn u_02() {
    use std::ops::{Add};

    fn add<T: Add<Output=T>>(i: T, j: T) -> T {
        i + j
    }

    fn x() {
        let (a, b) = (1.2, 3.4);
        let (x, y) = (10, 20);

        let c = add(a, b);
        let z = add(x, y);

        println!("{} + {} = {}", a, b, c);
        println!("{} + {} = {}", x, y, z);
    }

    //---------------------
    x();
}