

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
