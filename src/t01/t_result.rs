
#[test]
fn a_r_1() {
    fn f()->Result<i32,String>{
        Err("a".to_string())
    }

    fn b()->Result<i32,String>{
        Ok(f()?)
    }

    let r = b().unwrap_or(-1);
    println!("----t_result.rs-----{}--" , r);
}

