use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Data {
    id: i64,
    title: String,
}

#[test]
fn it_1() {
    let l: Vec<i64> = vec![1, 2, 3, 4];

    let a: Vec<Data> = l
        .iter()
        .map(|x| {
            let s = format!("{}_title", x);
            Data {
                id: x.clone(),
                title: s,
            }
        })
        .collect();

    println!("-----------{:?}-----------", a);
    a.iter().for_each(|x| println!("v: {:?}", x.clone()));
    println!("----------------------\n\n");
    let r = a.iter().find(|x| &x.id == &3);

    // let r = a.iter().contains();
    println!("-----------r {:?}-----------", r);
    let r = a.iter().any(|x| &x.id == &4);
    println!("-----------r {:?}-----------", r);
}

#[test]
fn v_m_1() {
    //---------------------
    let src = vec![1, 2, 3];
    let l: Vec<_> = src.iter().map(|x| x + 10).collect();
    println!("-----------{:#?}-----------", l);

    let b = src.iter().find(|&a| a == &2).is_some();
    println!("-----------{:?}-----------", b);
}

#[test]
fn ita_1() {
    //---------------------
    let a = vec![1, 3, 7, 10, 28, 40];
    println!("-----------{:?}-----------", a.iter().find(|&&x| x == 3));
    //
    let slice = ['r', 'u', 's', 't'];
    let mut iter = slice.windows(2);

    let a = iter.next().unwrap();
    println!("-----------{:?}-----------", a);

    let a = iter.next().unwrap();
    println!("-----------{:?}-----------", a);

    let a = iter.next().unwrap();
    println!("-----------{:?}-----------", a);
    //-------------------------------------
    let slice = ['r', 'u', 's', 't'];
    //
}
