fn print_test() {
    {
        println!("--------demo const declare -----------");
        const C: i32 = 4;
        println!(" {}", C);
        let mut c1: i32 = 10;
        println!(" mux c =  {} {}", C, c1);
    }
    //----------------------------------------
    println!("--------demo for  -----------");
    {
        for i in 0..5 {
            println!("i = {} ", i);
        }
    }
    {
        let a = [10, 20, 30, 40, 50];
        for i in a.iter() {
            println!("值为 : {}", i);
        }
    }
    //--------------------------------------
    {
        println!("--------demo while -----------");
        let mut i = 0;
        while i < 10 {
            // 循环体
            i += 1;
            println!(" i is  {}", i)
        }
    }

    {
        println!("--------demo loop -----------");
        let s = ['R', 'U', 'N', 'O', 'O', 'B'];
        let mut i = 0;
        let location = loop {
            let ch = s[i];
            if ch == 'O' {
                break i;
            }
            i += 1;
        };
        println!(" \'O\' 的索引为 {}", location);
    }

    //--owner------------------------------------
    {
        let x = 5;
        let y = x;
        println!("x is {}", x);
        println!("x is {}", x);
    }
    {
        println!("-------demo owner---------------");
        let s1 = String::from(" owner move hello");
        let s2 = s1;
        println!("owner demo {}", s2);
        // println!("owner demo {}", s1);
    }
    {
        println!("---------clone-------------");
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }
    {
        println!("------fn owner ship----------------");
        let s = String::from("hello");
        // s 被声明有效

        takes_ownership(s);
        // s 的值被当作参数传入函数
        // 所以可以当作 s 已经被移动，从这里开始已经无效

        let x = 5;
        // x 被声明有效

        makes_copy(x);
        // x 的值被当作参数传入函数
        // 但 x 是基本类型，依然有效
        // 在这里依然可以使用 x 却不能使用 s
        fn takes_ownership(some_string: String) {
            // 一个 String 参数 some_string 传入，有效
            println!("{}", some_string);
        } // 函数结束, 参数 some_string 在这里释放

        fn makes_copy(some_integer: i32) {
            // 一个 i32 参数 some_integer 传入，有效
            println!("{}", some_integer);
        } // 函数结束, 参数 some_string 是基本类型, 无需释放
    }

    //-------------zzz------------------------------
    {
        let s1 = gives_ownership();
        // gives_ownership 移动它的返回值到 s1

        let s2 = String::from("hello");
        // s2 被声明有效

        let s3 = takes_and_gives_back(s2);
        // s2 被当作参数移动, s3 获得返回值所有权
        // s3 无效被释放, s2 被移动, s1 无效被释放

        fn gives_ownership() -> String {
            let some_string = String::from("hello");
            // some_string 被声明有效

            return some_string;
            // some_string 被当作返回值移动出函数
        }

        fn takes_and_gives_back(a_string: String) -> String {
            // a_string 被声明有效

            a_string  // a_string 被当作返回值移出函数
        }
    }

    //-----reference and borrow--------------------------------------
    {
        let s1 = String::from("hello");
        let s2 = &s1;
        println!("s1 is {}, s2 is {}", s1, s2);
    }
    //-------------------------------------------
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
    }
    //---------------------------------------
    {
        let s1 = String::from("abc");
        let s2 = &s1;
        println!("-------------------{}-{}--", s1, s2);
    }
    {
        let mut s1 = String::from("run");
        // s1 是可变的

        let s2 = &mut s1;
        // s2 是可变的引用

        s2.push_str("oob");
        println!("{}", s2);
    }
}

