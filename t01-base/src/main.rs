use std::borrow::{Borrow, BorrowMut};
use std::ops::Add;

include!("t01-print-demo.rs");
include!("t02-slice-str-demo.rs");
include!("t03-map-demo.rs");
include!("t04-vec-demo.rs");
include!("t05-ownership-demo.rs");
include!("t07-struct.rs");


fn main() {
    println!("Hello, world!");
    print_test();
    slice_test();
    map_demo();
    ownership_demo();
    //
    vec_test();
    test_struct();
}




