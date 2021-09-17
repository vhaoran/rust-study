#[test]
fn t02() {
    slice_test();
}

fn slice_test() {
    {
        //-------------slice test 1------------------------------
        let s = String::from("0123456789");
        let part1 = &s[0..5];
        let part2 = &s[5..9];
        println!("{}={}+{}", s, part1, part2);
        println!("-------slice [..5]--{}-------------", &s[..5]);
        println!("-------slice [5..]--{}-------------", &s[5..]);
    }
    //-------------------------------------------
    {
        //-------------demo string append------------------------------
        println!("----------demo string append------------");
        let a = String::from("good bye");
        let mut c = String::new();
        c.push_str(a.as_str());
        c.push_str("---l am aa str");
        println!(" c =  {}", c);
        println!(" c =  {}", a);
    }
}

#[test]
fn a_str_mult() {
    println!(r#"--b"12上海"-------"#);
    let a = b"12";
    for i in a.iter(){
        println!("--------{}-" ,i);
    };
    //-------------------------------------
    println!(r#"--"12上海"-------"#);
    let a = "12上海";
    for i in a.as_bytes().iter(){
        println!("--------{}-" ,i);
    }
    //-----------aa--------------------------
    println!(r#"--"12上海"-------"#);
    let a = r"12上海";
    for i in a.as_bytes().iter(){
        println!("--------{}-" ,i);
    }
    //-----------aa--------------------------


}

#[test]
fn a_bool() {
    //---------------------
    let a: bool = false;
    println!("----t_slice_str.rs---b---{}--" ,a);
}

