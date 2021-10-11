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
    if a == b {
        println!("-----------a == b (i32)-----------");
    }
}

#[test]
fn for_1() {
    fn get() -> &'static str {
        "good"
    }
    fn get_i32() -> i32 {
        25i32
    }

    //---------------------
    for i in 0..1 {
        println!("-----------{}-----------", i);
        println!("-----------{}-----------", get());
        println!("-----------{}-----------", get_i32());
    }
}

#[test]
fn gb2312_disp() {
    let start = '中' as u32;
    // for i in 0xB0A1_u32..0xF7FE_u32 {
    for i in start..start + 2000 {
        let c = char::from_u32(i).unwrap_or('n');
        let s = format!("{}", c);
        println!("-----------{}-----------", s);
    }
}
