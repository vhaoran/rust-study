#[test]
fn str_3_t() {
    fn x() -> Option<()> {
        let s = "abc";
        if s == "abc" {
            println!("------------ok-------------");
            return Some(());
        }

        println!("------------not-------------");
        None
    }
    x();
}