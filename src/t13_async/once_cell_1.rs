
#[allow(unused_imports)]
#[allow(dead_code)]
use std::borrow::Borrow;
// use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use once_cell::sync::OnceCell;
// use time::NumericalStdDurationShort;
#[allow(unused_imports)]
#[allow(dead_code)]
static ONLY_ONE_CELL: OnceCell<u64> = OnceCell::new();

#[allow(unused_imports)]
#[allow(dead_code)]
fn set_v() {
    let i: u64 = 3;
    let _r = ONLY_ONE_CELL.set(i);
}

#[test]
fn once_cell_x_01() {
    set_v();

    // use async_std::{
    //     task,
    // };


    thread::spawn(move || {
        // This will wait for the parent thread to start receiving
        for i in 0..100000000 {
            let _a = ONLY_ONE_CELL.set(i as u64);

            if ONLY_ONE_CELL.get().is_some() {
                let a = ONLY_ONE_CELL.get().unwrap();
                println!(" is some:{} {} ", i, a.borrow())
            }
            // sleep(Duration::from_secs(2));
        }
    });
    thread::spawn(move || {
        // This will wait for the parent thread to start receiving
        for i in 0..100000000 {
            if ONLY_ONE_CELL.get().is_some() {
                let a = ONLY_ONE_CELL.get().unwrap();
                println!(" is some:{} {} ", i, a.borrow())
            }
            // sleep(Duration::from_secs(2));
        }
    });
    thread::spawn(move || {
        // This will wait for the parent thread to start receiving
        for i in 0..100000000 {
            if ONLY_ONE_CELL.get().is_some() {
                let a = ONLY_ONE_CELL.get().unwrap();
                println!(" is some:{} {} ", i, a.borrow().clone())
            }
            // sleep(Duration::from_secs(2));
        }
    });

    sleep(Duration::from_secs(100));
}
