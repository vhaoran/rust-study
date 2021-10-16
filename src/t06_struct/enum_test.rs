#[test]
fn enum_1() {
    //---------------------
    pub enum Data {
        A(i32, i32),
        B(String),
    }

    //
    let a = Data::A(3, 4);
    if let Data::A(i, j) = a {
        println!("-----------{},{}-----------", i, j);
    }
}
