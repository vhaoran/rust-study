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
    println!("-----------r {:?}-----------", r);
    let r = a.iter().any(|x| &x.id == &4);
    println!("-----------r {:?}-----------", r);
}

#[test]
fn v_m_1() {
    //---------------------
    let src = vec![1, 2, 3];
    let l:Vec<_> = src.iter().map(|x| x + 10).collect();

    println!("-----------{:#?}-----------", l);
}
