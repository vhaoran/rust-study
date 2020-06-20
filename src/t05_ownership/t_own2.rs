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


#[test]
fn own_test6() {
    own_6()
}

#[allow(dead_code)]
fn own_6() {
    let a = 1;
    let b = 2;

    let c = a + b;
    let d = c;

    let e = c;
    println!("e : {}", e);

    println!("a: {} b: {} c:{} d:{}", a, b, c, d);
    println!("a: {} b: {} c:{} d:{}", a, b, c, d);
    println!("a: {} b: {} c:{} d:{}", a, b, c, d);
}

#[test]
fn own_test7() {
    own_7()
}

#[allow(dead_code)]
fn own_7() {
    let mut a = String::from("aaaaa");
    let b = &a;
    let c = &b;


    println!("----------------------");
    println!("a: {}", a);
    println!("a: {} b : {} c: {} ", a, b, c);

    a.push_str(" this is append value");
    println!("---------------------{} ---", a);
}

//----------------------------------------
#[test]
fn own_test9() {
    own_9()
}

#[allow(dead_code)]
fn own_9() {
    println!("----------------------");
    let a = get();
    println!("---------------------{}", a);
    println!("----------------------");

    fn get() -> String {
        let a = String::from("aaa");
        let b = a;
        b
    }
}
//----------------------------------------

