use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use time::NumericalStdDurationShort;
use std::borrow::Borrow;

static only: OnceCell<u64> = OnceCell::new();


fn set_v() {
    let i: u64 = 3;
    only.set(i);
}

#[test]
fn once_cell_x_01() {
    set_v();

    use async_std::{
        task,
    };

    thread::spawn(move || {
        // This will wait for the parent thread to start receiving
        for i in 0..100000000 {
            only.set(i as u64);

            if only.get().is_some() {
                let a = only.get().unwrap();
                println!(" is some:{} {} ", i, a.borrow())
            }
            // sleep(Duration::from_secs(2));
        }
    });
    thread::spawn(move || {
        // This will wait for the parent thread to start receiving
        for i in 0..100000000 {
            if only.get().is_some() {
                let a = only.get().unwrap();
                println!(" is some:{} {} ", i, a.borrow())
            }
            // sleep(Duration::from_secs(2));
        }
    });
    thread::spawn(move || {
        // This will wait for the parent thread to start receiving
        for i in 0..100000000 {
            if only.get().is_some() {
                let a = only.get().unwrap();
                println!(" is some:{} {} ", i, a.borrow().clone())
            }
            // sleep(Duration::from_secs(2));
        }
    });

    sleep(Duration::from_secs(100));
}
