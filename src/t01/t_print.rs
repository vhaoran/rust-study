#[test]
fn t01() {
    print_test();
}

#[allow(dead_code)]
fn print_test() {
    {
        println!("--------demo const declare -----------");
        const C: i32 = 4;
        println!(" {}", C);
        let c1: i32 = 10;
        println!(" mux c =  {} {}", C, c1);
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
        println!("x is {}", y);
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
        let s2 = String::from("hello");
        // s2 被声明有效

        let _s3 = takes_and_gives_back(s2);
        // s2 被当作参数移动, s3 获得返回值所有权
        // s3 无效被释放, s2 被移动, s1 无效被释放

        #[allow(dead_code)]
        fn gives_ownership() -> String {
            let some_string = String::from("hello");
            // some_string 被声明有效

            return some_string;
            // some_string 被当作返回值移动出函数
        }

        fn takes_and_gives_back(a_string: String) -> String {
            // a_string 被声明有效

            a_string // a_string 被当作返回值移出函数
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

#[test]
fn tuple_1() {
    fn x() -> (bool, i32) {
        (true, 50)
    }
    let (a, b) = x();
    println!("------------{}  {}-------------", a, b);
}

#[test]
fn loop_0() {
    //---------------------
    let mut i = 0;
    let x = loop {
        i += 1;
        if i < 10 {
            break i;
        }
    };

    //---------------------------------------
    println!("----t_print.rs--------");
    println!("----t_print.rs---{}-----", x);
}

#[test]
fn f_1_1() {
    //---------------------
    let f = 1.23345f64;
    println!("-----------{:0.2}-----------", f);
}

#[test]
fn let_1() {
    let a = "s".to_string();
    let b = "s".to_string();
    if a.as_bytes().eq(b.as_bytes()) {
        println!("-----------equal-----------");
    }
    println!("-----------end-----------");

    let i = 36i128.pow(15);
    println!("-------sec:----{}-----------", i);
    let year = i / (86400 * 365 * 1000000000);
    println!("---------year:--{}-----------", year);
    println!("---------year-secs:--{}-----------", 86400 * 365);
}

#[test]
fn a_z_1() {
    //---------------------
    for c in 'A'..'Z' {
        println!("-----------{}-----------", c);
        println!("-----to_string------{}-----------", c.to_string());
    }
}

#[test]
fn std_in_1() {
    let mut line = String::new();
    println!("请输入你的名字:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    let s = line.replace("\n", "");
    println!("你好 , /{}/", s);
    println!("读取的字节数为：{}", b1);
    println!(" s.len()：{}", s.len());
    println!(" line.len()：{}", line.len());
}
