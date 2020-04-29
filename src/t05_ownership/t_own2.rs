//-------------------------------------------
#[test]
fn own_test2() {
    own_2();
}

#[allow(dead_code)]
fn own_2() {
    let s = {
        String::from("aaaa")
    };

    println!("-------------------{}---", "");
    println!("s = {}", s);
}


//-------------------------------------------
#[test]
fn own_test3() {
    own_3();
}

#[allow(dead_code)]
fn own_3() {
    let s = {
        "aaaa"
    };

    println!("-------------------{}---", "");
    println!("s = {}", s);
    println!("s = {}", s);
    println!("s = {}", s);
}

//-------------------------------------------
//-------------------------------------------
#[test]
fn own_test4() {
    own_4();
}

#[allow(dead_code)]
fn own_4() {
    let s = f();
    fn f() -> String {
        let s = String::from("hello");
        s
    }

    println!("-------------------{}---", "");
    println!("s = {}", s);
    println!("s = {}", s);
    println!("s = {}", s);
}

//-------------------------------------------
#[test]
fn own_test5() {
    own_5();
}

#[allow(dead_code)]
fn own_5() {
    let s = f();

    struct SSS {
        a: i32,
        b: i32,
    }

    fn f() -> SSS {
        SSS {
            a: 3,
            b: 4,
        }
    }

    println!("-------------------{}---", "");
    println!("s = {}", s.a);
    println!("s = {}", s.b);
}
//-------------------------------------------

