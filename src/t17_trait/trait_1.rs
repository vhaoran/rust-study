#[test]
fn tt_01() {
    pub trait XTrait {
        fn abs(&self) -> usize;
        fn abt(&self) -> usize;
    }

    pub struct XTChild {
        x: u8,
        y: u8,
    }
    pub struct XTChildA {
        x: u8,
        y: u8,
    }

    impl XTChild {
        fn print(&self) {
            println!("----trait_1.rs---x:{},y:{}-----", self.x, self.y);
        }
    }

    impl XTChildA {
        fn print(&self) {
            println!("----trait_1.rs---x:{},y:{}-----", self.x, self.y);
        }
    }

    impl From<&XTChildA> for XTChild {
        fn from(src: &XTChildA) -> Self {
            XTChild {
                x: src.x,
                y: src.y,
            }
        }
    }

    impl XTrait for XTChild {
        fn abs(&self) -> usize {
            println!("----enter abs--------");
            6
        }

        fn abt(&self) -> usize {
            println!("----enter abt--------");
            8
        }
    }
    //-------------------------------------
    let a = XTChild { x: 3, y: 4 };
    a.abs();
    a.abt();
    let b = XTChildA { x: 5, y: 10 };

    let bb = XTChild::from(&b);

    println!("----trait_1.rs--------");
    a.print();
    b.print();
    bb.print();
}





