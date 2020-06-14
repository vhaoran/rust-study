//use std::string::ToString;
use std::ops::Add;

#[derive(Debug)]
struct Abc {
    a: String,
    b: String,
}

struct Color(i8, i8, i8);

impl Abc {
    fn to_string(&self) -> String {
        //self.a.clone().add(self.b.clone().as_ref())
        self.a.clone().add(self.b.clone().as_ref())
    }
}

#[test]
fn stru_test_1() {
    let c = Color(1, 2, 3);
    println!("color: {} {} {}", c.0, c.1, c.2);
    let mut data = Abc {
        a: String::from("aaa"),
        b: String::from("bbbbb"),
    };
    println!("abc: {:?}", data);
    println!("abc: {:?}", data);
    println!("abc: {} {}", data.a, data.b);
    data.a = String::from("test_update _a");
    println!("abc: {} {}", data.a, data.b);
    //-------------------------------------------------------------------
    println!("----------------------");
    println!("abc: {} ", data.to_string());
}



