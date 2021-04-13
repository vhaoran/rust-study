#[test]
fn hashset_t_01() {
    // use std::cmp::Ordering;
    use std::collections::HashSet;
    use std::fmt::{Debug, Formatter};
    use std::hash::{Hash, Hasher};

    fn func(hss: HashSet<Box<dyn MyTrait>>) {
        let hs = HashSet::new();
        hs.union(&hss);
    }
    trait MyTrait {
        fn method(&self);

        fn my_hash(&self) -> i32;

        fn my_eq(&self, other: &dyn MyTrait) -> bool;
        fn my_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
    }

    struct MyTraitImpl;

    impl MyTrait for MyTraitImpl {
        fn method(&self) {
            println!("hello")
        }

        fn my_hash(&self) -> i32 {
            0
        }

        fn my_eq(&self, other: &dyn MyTrait) -> bool {
            println!("----hashset_1.rs-----{}---" ,other.my_hash());
            true
        }
        fn my_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str("hello")
        }
    }

    impl Debug for dyn MyTrait {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            self.my_fmt(f)
        }
    }
    impl Hash for dyn MyTrait {
        fn hash<H: Hasher>(&self, state: &mut H) {
            state.write_i32(self.my_hash())
        }
    }

    impl PartialEq for dyn MyTrait {
        fn eq(&self, other: &Self) -> bool {
            self.my_eq(other)
        }
    }

    impl Eq for dyn MyTrait {}

    fn x() {
        let mut hs: HashSet<Box<dyn MyTrait>> = HashSet::new();
        hs.insert(Box::new(MyTraitImpl));
        hs.insert(Box::new(MyTraitImpl));
        hs.insert(Box::new(MyTraitImpl));
        hs.insert(Box::new(MyTraitImpl));
        println!("{:?}", hs);
        for h in hs.iter() {
            println!("{:?}", h);
        }
        func(hs);
    }
    //-------------------------------------
    x();
}
