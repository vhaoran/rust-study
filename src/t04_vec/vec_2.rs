use std::ops::Add;
// use std::io::Read;

#[test]
fn ve_01() {
    let s = "abc中国";
    let l = s.as_bytes();
    //-------------------------------------
    let mut i = 0;
    for v in l.iter() {
        i = i.add(1);
        println!("----vec_2.rs---{}---{}--", v, i);
    }

    let s = std::str::from_utf8(l.iter().as_slice());
    match s {
        Ok(ss) => {
            println!("---{}------", ss);
            ()
        }
        _ => (),
    }
    // println!("----vec_2.rs---{}-----", s);
}

#[test]
fn t_vec_find() {
    //---------------------
    let mut l: Vec<i32> = Vec::new();
    l.push(1);
    l.push(4);
    l.push(5);
    let r = l.iter().find(|&&x| x == 3);
    println!("----------------------");
    println!("fihnd 3: {:?}", r);
    let r = l.iter().find(|&&x| x == 4);
    println!("-----------aa-----------");
    println!("fihnd 4: {:?}", r);
}

#[test]
fn v_7() {
    let mut l = vec![1, 2, 3, 4];

    let i = 1;
    l.insert(i, -1);
    println!("-----------{:?}-----------", l);
    l.remove(i + 1);
    println!("--remvoe ---------{:?}-----------", l);
    l[i] = 44444;
    println!("--remvoe ---------{:?}-----------", l);
}

#[test]
fn v_8() {
    let mut l = vec![1, 2, 3, 4];

    let i = 1;
    let r = l.remove(0);
    println!("-----------0: {:?}--/{:?}---------", r, l);
    let r = l.remove(0);
    println!("-----------0: {:?}--/{:?}---------", r, l);
    let r = l.remove(0);
    println!("-----------0: {:?}--/{:?}---------", r, l);
}

#[test]
fn split_1() {
    //---------------------
    let mut text = "aaa***tail\n222\n333\n".to_string();
    for i in 0..5 {
        text = text.replace("**", "*").to_string();
    }

    let l: Vec<&str> = text.split("\n").collect();
    if l.len() == 0 {
        return;
    }
    //
    let s = l.get(0).unwrap();
    let l: Vec<_> = s.split("*").collect();
    if l.len() < 2 {
        return;
    }
    let head = l.get(0).unwrap().clone();
    let tail = l.get(1).unwrap().clone().replace("*", "");
    println!("-----------{head} {tail}-----------");
}
