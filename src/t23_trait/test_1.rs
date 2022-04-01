use iron::error::HttpError::Ssl;

pub trait X<T> {
    fn my_name(&self) -> String {
        "my_name".to_string()
    }
}

#[derive(Debug)]
pub struct Xa {
    pub id: String,
    pub name: String,
}

impl X for Xa {}

#[test]
fn a_1() {
    //---------------------
    let src = Xa {
        id: "id".to_string(),
        name: "name".to_string(),
    };

    println!("-----------{}-----------", src.my_name());
}
