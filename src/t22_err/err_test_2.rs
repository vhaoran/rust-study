

#[test]
fn t_e_2() {
    let p = "./readme.md";
    let a = file_len(p.to_string());
    println!("----err_test_2.rs---aa----{:#?}-"  ,a);
}

fn file_len(p :String)->Result<usize,String> {
    let s = std::fs::read_to_string(p.as_str())
    .map_err(|e|e.to_string())?;

    Ok(s.len())
}

