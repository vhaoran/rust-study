

#[test]
fn a_0a() {
    let p = "./readme.md";
    let a = file_len(p.to_string());
    println!("----err_test_2.rs---a----{:#?}-"  ,a);

}

fn file_len(p :String)->Result<usize,String> {
    std::fs::read_to_string(p.as_str())
    .map_err(|e|e.to_string())
        .and_then(|s|Ok(s.len()))
}
