#[allow(unused_imports)]
#[allow(dead_code)]

use std::collections::HashMap;

#[test]
fn test_str() {
    let a = "aaaa".to_string();
    let b = a.clone();
    let c = b.clone();

    println!("aa: {} b: {}", a, b);
    println!("aa: {} b: {}", a, b);
    println!("aa: {} b: {}", a, b);
    println!("aa: {} b: {}", a, b);
    println!("s: {} ", c);
    println!("s: {} ", c);
    println!("s: {} ", c);
    println!("s: {} ", c);
}

#[test]
fn str_owner_1() {
    let a = "aaa";
    let b = a;
    let c = b;
    println!("------------{}-------------", a);
    println!("------------{}-------------", b);
    println!("------------{}-------------", c);
}

#[test]
fn int_owner_1() {
    let a = 111;
    let b = a + 1;
    let c = b + 2;

    println!("------------{}-------------", a);
    println!("------------{}-------------", b);
    println!("------------{}-------------", c);
}

#[test]
fn str_2() {
    let s = "abc";
    let b = s.starts_with("aa");
    println!("------------{}-------------", b);
    println!("------contains Of-b-----{}-------------",
             s.contains("aa"));

    println!("------find Of-aa---index--{}-------------",
             s.find("aa").unwrap());
    println!("------find Of-ba--index---{}-------------",
             match s.find("ba") {
                 Some(v) => format!("{}", v),
                 _ => "not found".to_string(),
             });
}

#[test]
fn str_split_1() {
    let s = "/ws?jwt=test|1&v2=2&v3=3";
    let l: Vec<_> = s.split("?").collect();
    for i in l.iter() {
        println!("------------{}-------------", i);
    }
}

#[test]
fn t_get_params() {
    let url = "/ws?jwt=test|1&v2=2&v3=3";
    let m = get_params_of_url(url).unwrap();
    for (k, v) in m.iter() {
        println!("------------{} {}-------------", k, v);
    }

    let pre = get_prefix_of_url(url);
    println!("-----------{}--------------", pre.unwrap());
}

#[test]
#[allow(unused_imports)]
#[allow(dead_code)]
fn url_pwd_get_1() {
    let url = "ws://whr:123@127.0.0.1:9999";
    let z = get_user_pwd_of_url(url);
    println!("-------------------------");
    match z {
        Some((a, b)) => println!(" {}:{}", a, b),
        _ => println!("no match!"),
    }
}

#[allow(unused_imports)]
#[allow(dead_code)]
fn get_user_pwd_of_url(url: &str) -> Option<(String, String)> {
    let l: Vec<_> = url.split("//").collect();
    if l.len() < 2 {
        return None;
    }

    let l: Vec<_> = l[1].split("@").collect();
    if l.len() < 2 {
        return None;
    }
    let l: Vec<_> = l[0].split(":").collect();
    if l.len() < 2 {
        return None;
    }

    Some((l[0].to_string(), l[1].to_string()))
}

#[allow(unused_imports)]
#[allow(dead_code)]
fn get_prefix_of_url(url: &str) -> Option<String> {
    let l: Vec<_> = url.split("?").collect();
    if l.len() < 2 {
        return Some(url.to_string());
    }
    Some(l[0].to_string())
}

#[allow(unused_imports)]
#[allow(dead_code)]
fn get_params_of_url(url: &str) -> Option<HashMap<String, String>> {
    let l: Vec<_> = url.split("?").collect();
    if l.len() < 2 {
        return None;
    }

    let ll: Vec<_> = l[1].split("&").collect();

    let mut m: HashMap<String, String> = HashMap::new();
    let mut b = false;
    for i in ll.iter() {
        let l2: Vec<_> = i.split("=").collect();
        if l2.len() == 2 {
            m.insert(l2.get(0).unwrap().to_string(),
                     l2.get(1).unwrap().to_string(),
            );
            b = true;
        }
    }

    match b {
        true => Some(m),
        _ => None,
    }
}
