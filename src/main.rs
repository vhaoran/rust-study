use std::borrow::{Borrow, BorrowMut};
use std::ops::Add;
include!("print-demo.rs");
include!("slice-str-demo.rs");
include!("map-demo.rs");
include!("vec-demo.rs");
include!("ownership-demo.rs");


fn main() {
    println!("Hello, world!");
    print_test();
    //
    slice_test();
    //
    map_demo();
    //
    vec_test();
    //
    ownership_demo();
}




