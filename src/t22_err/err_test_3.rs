#[test]
fn t_err_3() {
    fn r() -> Result<(), String> {
        a()?;
        b()?;
        Ok(())
    }

    fn r_1() -> Result<i32, String> {
        a()?;
        b()?;
        Ok(1i32)
    }

    fn a() -> Result<u32, String> {
        Ok(1u32)
    }
    fn b() -> Result<i32, String> {
        Ok(1i32)
    }

    r();
    println!("----() return passed-----");
    r_1();
    println!("----i32 return passed-----");
}

#[test]
fn a_t_map() {
    //---------------------
    fn f() -> Option<i32> {
        Some(5)
    }

    let a = f().map(|x| x + 1).unwrap_or(-1);

    println!("----err_test_3.rs---a----{}-", a);
}

#[test]
fn a_filter_option() {
    fn f() -> Option<i32> {
        Some(5)
    }

    let a = f().filter(|x| return *x > 3);
    println!("----err_test_3.rs---a----{:?}-", a);
}

#[test]
fn t_opt_1() {
    let a: Option<i32> = None;
    let r = a.map_or_else(|| Err("error ".to_string()), |_x| Ok("good".to_string()));
    println!("result: {:?}", r);
}

#[test]
fn err_link_1() {
    let r: Result<i32, String> = Ok(3);
    r.map_err(|e| {
        println!("error is occur {}", e.to_string());
    })
    .map(|_| {
        println!("ok doing....");
    });
}

#[test]
fn str_1() {
    let s = "good morning";
    let c = s.replace("g", "a");
    println!("-----------a-----------");

    println!(" {}", s);
    println!(" {}", c);
}

#[test]
fn opt_2() {
    //---------------------
    let s = r#" try after 256" "#;
    let r = s.rfind("after");
    println!("-----------S----{}----", s);
    println!(" index : {:?}", r);

    let v: Vec<&str> = s.split(" ").collect();
    println!("-----------r----{:?}----", v);
    for s in v.iter().rev() {
        let str = s.clone().trim().replace("\"", "").replace("(", "");
        println!(" {:?}", str);
        let n_str: Vec<_> = str.matches(char::is_numeric).collect();
        println!("     n_str {:?}", n_str);
        if n_str.len() > 0 {
            if str.parse::<i32>().unwrap_or(0) > 0 {
                println!("           *** {}", str);
            }
        }
    }
}
