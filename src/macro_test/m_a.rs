#[test]
fn m_1() {
    macro_rules! a {
        () => {
            3
        };
    }
    println!("-----------aa-----------");
    println!(" {:?}", a!());
}

#[test]
fn m_2() {
    macro_rules! f {
        ($a:ident) => {
            fn $a(i: i32) -> i32 {
                i + 100
            }
        };
    }
    f!(test_fn);
    let r = test_fn(5);
    println!("-----------aa-----------");
    println!(" {}", r);
    let s = stringify!(563);
    println!("---------------s: {}-------", s);
}

#[test]
fn m_4() {
    macro_rules! c {
        ($a:expr ,gt $b:expr) => {
            println!("-------------{}-> {}--------", $a, $b);
        };
    }
    println!("-----------aa-----------");
    println!(" {:?}", c!(3 ,gt 4));
}
