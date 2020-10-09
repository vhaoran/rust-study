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

#[test]
fn str_test_contains() {
    let url = "/ws?abc=3";
    if url.contains("/ws") {
        println!("------------con ok-------------");
    }
    println!("-------------------------");
}


#[test]
fn t_option_2() {
    let a = Some(Some(1));
    match a {
        Some(Some(v)) => println!("----------{}---------------", v),
        _ => println!("------------none-------------"),
    }
    println!("-------------------------");
}

