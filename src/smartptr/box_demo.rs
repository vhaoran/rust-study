// use std::alloc::dealloc;
// use std::intrinsics::atomic_xadd;

#[allow(dead_code)]
fn box_demo_1() {
    let a = Box::new(3);
    //-------------------------------------------------------------------

    println!("----------------------");
    println!("{}", a);
    println!("----------------------");
    let b = a.clone();
    println!("b = {}", b);
    println!("a = {}", a);
}

#[allow(dead_code)]
fn box_demo_2() {
    let x = 5;
    let y = Box::new(x);

    println!("----------------------");
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("----------------------");
}


fn box_demo_3() {
    struct Abc {
        a: i8,
        b: i8,
    }

    let a: Box<Abc> = Box::new(Abc {
        a: 1,
        b: 2,
    });
    //-------------------------------------------------------------------

    println!("-------a:{},b:{}-----------", a.a, a.b);
}

#[test]
fn box_test1() {
    box_demo_1();
}


#[test]
fn box_test2() {
    box_demo_2();
}


//----------------------------------------
#[test]
fn box_test3() {
    box_demo_3();
}

//----------------------------------------


