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

#[test]
fn c_2() {
    //---------------------
    let s = "abc中国|/,.";
    //
    let l = s.chars();
    let mut count = 0usize;
    for v in l {
        if (v >= 'a' && v <= 'z') || (v >= 'A' && v <= 'Z') || "/|,.;".contains(v) {
            count += 1;
            continue;
        }
        count += 2;
    }
    println!("-----------c {}-----------", count);
    println!("-----------s: {}-----------", s);
}

#[test]
fn eq_1() {
    //---------------------
    let s = "a".to_string();
    if s == "a".to_string() {
        println!("-----------eq-----------");
    }
    println!("----------------------");
    let s = "a";
    if s == "a" {
        println!("-----------eq-----------");
    }
    println!("----------------------");

    let a = 1i32;
    let b = 1i32;
    if a == b{
        println!("-----------a == b (i32)-----------");
    }
}
