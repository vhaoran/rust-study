

use std::fmt::{Debug, Display};

pub struct Verify{
   on_err_stop:bool,
   errs: Vec<String>,
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
impl Clone for Verify{
   fn clone(&self) -> Self {
      Verify{
         self:on_err_stop,
         errs:self.errs.clone(),
      }
   }
}


impl Verify{
   pub fn new(on_err_stop:bool)->Verify{
      Verify{
         errs:std::vec::Vec::new(),
         on_err_stop:on_err_stop,
      }
   }

   pub fn gt<T>(&self,info:&str,n1:&T,n2:&T)->Verify
      where T: PartialOrd  {
      use std::cmp::Ordering;
      let mut r = self.clone();

      if n1.le(n2){
          r.errs.push(info.to_string());
      }
      r
   }




}

#[test]
fn v_1() {
    let r = Verify::new(true)
        .gt("" ,&1u8 ,&0u8)
        .gt("int should greater than 10" ,&1u8 ,&10u8)
        .gt("float should greater than 10" ,&1f32 ,&10f32);


    println!("-----verifys.rs---------{:?}--" ,r);
}

