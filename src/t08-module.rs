//mod test1;




mod test1;
pub use crate::test1::a;
//mod test1;

fn module_demo1() {
    println!("-------demo module---------------");
    test1::hello();
    test1::a::hello();
    test1::b::hello();

    {
        //
        //-------------------------------------------
        println!("-----array->rev----------------");
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    }
}

#[test]
fn aa(){
    //-------------------------------------------
    println!("----------------------");
    println!("abc");
    println!("----------------------");
}
