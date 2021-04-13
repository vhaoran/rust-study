mod trait_1;
mod trait_2;


#[test]
fn trait_t_1() {
    //----------------------------------------
    trait Calc {
        fn add(&self, i: i32) -> i32;
    }
    //----------------------------------------
    #[derive(Copy, Clone)]
    struct IntAdd {
        i: i32,
    }

    //----------------------------------------
    impl Calc for IntAdd {
        fn add(&self, i: i32) -> i32 {
            self.i + i
        }
    }

    //----------------------------------------
    impl IntAdd {
        fn new(i: i32) -> IntAdd {
            IntAdd { i: i }
        }
    }

    //----------------------------------------
    println!("---------------------");
    let obj = IntAdd::new(1);
    let i = obj.add(1);
    println!("----{}----", i);

    let c = obj.clone();
    let i = c.add(1);
    println!("----{}----", i);
}

