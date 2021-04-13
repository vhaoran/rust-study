



#[test]
fn gen_2() {
    pub trait ICopyTo{
        fn copy_to(&self)->Self;
    }

    impl ICopyTo for String{
        fn copy_to(&self) -> Self {
          self.clone()
        }
    }

    let a = "abc".to_string().copy_to();
    println!("----trait_2.rs----{:#?}----" ,a);
}
