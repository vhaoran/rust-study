fn ownership_demo() {
    {
        let a = 2;
        let mut c = 3;
        ch(&mut c);
        println!("### c is {} ,no changed", c);
        c = 5;
        println!("### c is {} change", c);
        println!("------changed----------------");
        changed(&mut c);
        println!(" c is  {} ", c);

        fn ch(x: &i32) {
            println!(" enter ch fn->x = {}",x);
        }

        fn changed(x: &mut i32) {
            let a = 33333;
            *x = a;
        }
    }
}