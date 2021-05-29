


use std::fmt::{Debug, Display};

pub trait NotEmpty{
    fn not_empty<T>(&self,str:&str,data:Option<T>)->Self;
}

#[derive(Clone)]
pub struct Verify{
   on_err_stop:bool,
   errs: Vec<String>,
}

impl NotEmpty for Verify{
   fn not_empty<T>(&self,str:&str,data:Option<T>)->Self{
      let mut r = self.clone();
      if data.is_none(){
         r.errs.push(str.to_string());
      }
      r
   }
}

impl Display for Verify{
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "({}, {:#?})", self.on_err_stop,self.errs)
   }
}

impl Debug for Verify{
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_tuple("")
          .field(&self.on_err_stop)
          .field(&self.errs)
          .finish()
   }
}

impl Verify{
   pub fn new(on_err_stop:bool)->Verify{
      Verify{
         errs:std::vec::Vec::new(),
         on_err_stop,
      }
   }

   pub fn gt<T>(&self,info:&str,n1:&T,n2:&T)->Verify
      where T: PartialOrd  {
      use std::cmp::Ordering;
      let mut r = self.clone();

      if n1.le(n2){
          r.errs.push(info.to_string());
      }
      r.to_owned()
   }

   pub fn gte<T>(&self,info:&str,n1:&T,n2:&T)->Verify
      where T: PartialOrd  {
      use std::cmp::Ordering;
      let mut r = self.clone();

      if n1.lt(n2){
          r.errs.push(info.to_string());
      }
      r.to_owned()
   }

   pub fn lt<T>(&self,info:&str,n1:&T,n2:&T)->Verify
      where T: PartialOrd  {
      use std::cmp::Ordering;
      let mut r = self.clone();

      if n1.ge(n2){
          r.errs.push(info.to_string());
      }
      r.to_owned()
   }
   pub fn lte<T>(&self,info:&str,n1:&T,n2:&T)->Verify
      where T: PartialOrd  {
      use std::cmp::Ordering;
      let mut r = self.clone();

      if n1.gt(n2){
          r.errs.push(info.to_string());
      }
      r.to_owned()
   }

   pub fn fn_true(&self,info:&str,b :bool)->Verify {
      let mut r = self.clone();
      if !b{
         r.errs.push(info.to_string());
      }
      r.to_owned()
   }

   pub fn fn_no_err(&self,input :Result<(),String>)->Verify {
      let mut r = self.clone();
      if input.is_err() {
         if let Err(e) = input{
            r.errs.push(e);
         }
      }
      r.to_owned()
   }

   fn func<F>(&self,f: F) -> Verify
      where F : Fn() -> Result<(),String> {
      let mut r = self.clone();
      if let Err(e) = f(){
         r.errs.push(e);
      }
      r.to_owned()
   }

}

#[test]
fn v_1() {
    let r = Verify::new(false)
        .gt("" ,&1u8 ,&0u8)
        .gt("int should greater than 10" ,&1u8 ,&10u8)
        .gt("float should gre ater than 10" ,&1f32 ,&10f32)
        .fn_no_err(Err("something is wrong!!!!".to_string()))
        ;

    println!("-----verifys.rs---------{:?}--" ,r);
}

