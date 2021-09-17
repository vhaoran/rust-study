#[test]
fn t05() {
    ownership_demo();
}

#[allow(dead_code)]
fn ownership_demo() {
    {
        // let aa = 2;
        let mut c = 3;
        ch(&mut c);
        println!("### c is {} ,no changed", c);
        c = 5;
        println!("### c is {} change", c);
        println!("------changed----------------");
        changed(&mut c);
        println!(" c is  {} ", c);

        fn ch(x: &i32) {
            println!(" enter ch fn->x = {}", x);
        }

        fn changed(x: &mut i32) {
            let a = 33333;
            *x = a;
        }
    }
}