

#[test]
fn t_err_3() {
    fn r()->Result<(),String>{
       a()?;
       b()?;
        Ok(())
    }

    fn r_1()->Result<i32,String>{
       a()?;
       b()?;
       Ok(1i32)
    }

    fn a()->Result<u32,String>{
        Ok(1u32)
    }
    fn b()->Result<i32,String>{
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
    fn f()->Option<i32>{
        Some(5)
    }

    let a = f().map(|x|x+1).unwrap_or(-1);

    f().map_or()
    println!("----err_test_3.rs---a----{}-" ,a);
}

#[test]
fn a_filter_option() {
    fn f()->Option<i32>{
        Some(5)
    }

    let a = f().filter(|x|return *x > 3);
    println!("----err_test_3.rs---a----{:?}-" ,a);
}
